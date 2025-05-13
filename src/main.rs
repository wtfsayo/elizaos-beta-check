use prettytable::{row, Table};
use reqwest::Client;
use serde_json::Value;
use std::{error::Error, fs::File, io::Write, time::Duration};
use tokio::time::sleep;


const PACKAGES: &[&str] = &[
    "@elizaos/core",
    "@elizaos/adapter-postgres",
    "@elizaos/adapter-redis",
    "@elizaos/adapter-sqlite",
    "@elizaos/adapter-sqljs",
    "@elizaos/adapter-supabase",
    "@elizaos/client-auto",
    "@elizaos/client-direct",
    "@elizaos/client-farcaster",
    "@elizaos/client-github",
    "@elizaos/client-telegram",
    "@elizaos/client-twitter",
    "@elizaos/plugin-0g",
    "@elizaos/plugin-bootstrap",
    "@elizaos/plugin-coinbase",
    "@elizaos/plugin-conflux",
    "@elizaos/plugin-evm",
    "@elizaos/plugin-goat",
    "@elizaos/plugin-icp",
    "@elizaos/plugin-image-generation",
    "@elizaos/plugin-node",
    "@elizaos/plugin-solana",
    "@elizaos/plugin-starknet",
    "@elizaos/plugin-tee",
    "@elizaos/plugin-trustdb",
    "@elizaos/plugin-video-generation",
    "@elizaos/plugin-web-search",
    "@elizaos/plugin-whatsapp",
    "@elizaos/agent",
    "@elizaos/client-discord",
    "@elizaos/plugin-tee-log",
    "@elizaos/plugin-sgx",
    "@elizaos/adapter-pglite",
    "@elizaos/client-lens",
    "@elizaos/client-slack",
    "@elizaos/plugin-akash",
    "@elizaos/plugin-anyone",
    "@elizaos/plugin-3d-generation",
    "@elizaos/plugin-allora",
    "@elizaos/plugin-abstract",
    "@elizaos/plugin-arthera",
    "@elizaos/plugin-aptos",
    "@elizaos/plugin-asterai",
    "@elizaos/plugin-avalanche",
    "@elizaos/plugin-binance",
    "@elizaos/plugin-autonome",
    "@elizaos/plugin-avail",
    "@elizaos/plugin-coingecko",
    "@elizaos/plugin-coinmarketcap",
    "@elizaos/plugin-cosmos",
    "@elizaos/plugin-cronoszkevm",
    "@elizaos/plugin-depin",
    "@elizaos/plugin-echochambers",
    "@elizaos/plugin-flow",
    "@elizaos/plugin-genlayer",
    "@elizaos/plugin-fuel",
    "@elizaos/plugin-giphy",
    "@elizaos/plugin-goplus",
    "@elizaos/plugin-hyperliquid",
    "@elizaos/plugin-intiface",
    "@elizaos/plugin-irys",
    "@elizaos/plugin-gitbook",
    "@elizaos/plugin-letzai",
    "@elizaos/plugin-massa",
    "@elizaos/plugin-movement",
    "@elizaos/plugin-nft-generation",
    "@elizaos/plugin-multiversx",
    "@elizaos/plugin-obsidian",
    "@elizaos/plugin-near",
    "@elizaos/plugin-opacity",
    "@elizaos/plugin-open-weather",
    "@elizaos/plugin-primus",
    "@elizaos/plugin-quai",
    "@elizaos/plugin-rabbi-trader",
    "@elizaos/plugin-spheron",
    "@elizaos/plugin-stargaze",
    "@elizaos/plugin-story",
    "@elizaos/plugin-sui",
    "@elizaos/plugin-tee-marlin",
    "@elizaos/plugin-thirdweb",
    "@elizaos/plugin-ton",
    "@elizaos/plugin-tts",
    "@elizaos/plugin-twitter",
    "@elizaos/plugin-zksync-era",
    "@elizaos/client-deva",
    "@elizaos/client-xmtp",
    "@elizaos/client-alexa",
    "@elizaos/client-instagram",
    "@elizaos/adapter-mongodb",
    "@elizaos/client-telegram-account",
    "@elizaos/client-simsai",
    "@elizaos/client-eliza-home",
    "@elizaos/adapter-qdrant",
    "@elizaos/plugin-0x",
    "@elizaos/plugin-agentkit",
    "@elizaos/plugin-arbitrage",
    "@elizaos/plugin-ankr",
    "@elizaos/plugin-birdeye",
    "@elizaos/plugin-apro",
    "@elizaos/plugin-b2",
    "@elizaos/plugin-bittensor",
    "@elizaos/plugin-bnb",
    "@elizaos/plugin-chainbase",
    "@elizaos/plugin-cronos",
    "@elizaos/plugin-dcap",
    "@elizaos/plugin-dexscreener",
    "@elizaos/plugin-devin",
    "@elizaos/plugin-di",
    "@elizaos/plugin-email-automation",
    "@elizaos/plugin-email",
    "@elizaos/plugin-dkg",
    "@elizaos/plugin-football",
    "@elizaos/plugin-form",
    "@elizaos/plugin-gitcoin-passport",
    "@elizaos/plugin-gelato",
    "@elizaos/plugin-hyperbolic",
    "@elizaos/plugin-imgflip",
    "@elizaos/plugin-holdstation",
    "@elizaos/plugin-initia",
    "@elizaos/plugin-iq6900",
    "@elizaos/plugin-lens-network",
    "@elizaos/plugin-lightning",
    "@elizaos/plugin-lit",
    "@elizaos/plugin-injective",
    "@elizaos/plugin-mina",
    "@elizaos/plugin-moralis",
    "@elizaos/plugin-mind-network",
    "@elizaos/plugin-news",
    "@elizaos/plugin-nft-collections",
    "@elizaos/plugin-omniflix",
    "@elizaos/plugin-openai",
    "@elizaos/plugin-quick-intel",
    "@elizaos/plugin-router-nitro",
    "@elizaos/plugin-sei",
    "@elizaos/plugin-solana-agent-kit",
    "@elizaos/plugin-pyth-data",
    "@elizaos/plugin-squid-router",
    "@elizaos/plugin-solana-v2",
    "@elizaos/plugin-suno",
    "@elizaos/plugin-tee-verifiable-log",
    "@elizaos/plugin-trikon",
    "@elizaos/plugin-zerion",
    "@elizaos/plugin-zilliqa",
    "@elizaos/plugin-udio",
    "@elizaos/plugin-nvidia-nim",
    "@elizaos/plugin-ethstorage",
    "@elizaos/core-patched",
    "@elizaos/plugin-desk-exchange",
    "@elizaos/plugin-edwin",
    "@elizaos/cli-v2",
    "@elizaos/plugin-telegram",
    "@elizaos/cli",
    "@elizaos/plugin-discord",
    "@elizaos/plugin-elevenlabs",
    "@elizaos/plugin-starter",
    "@elizaos/project-starter",
    "@elizaos/plugin-anthropic",
    "@elizaos/plugin-local-ai",
    "@elizaos/plugin-sql",
    "@elizaos/the-org",
    "@elizaos/plugin-browser",
    "@elizaos/plugin-video-understanding",
    "@elizaos/plugin-pdf",
    "@elizaos/plugin-storage-s3",
    "@elizaos/app",
    "@elizaos/plugin-farcaster",
    "@elizaos/plugin-groq",
    "@elizaos/plugin-redpill",
    "@elizaos/plugin-ollama",
    "@elizaos/plugin-venice",
];



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut table = Table::new();
    let mut csv_data = vec!["Package,BetaVersion".to_string()];

    table.add_row(row!["Package", "BetaVersion"]);

    for (i, pkg) in PACKAGES.iter().enumerate() {
        let encoded = urlencoding::encode(pkg);
        let url = format!("https://registry.npmjs.org/{}", encoded);

        let res = client.get(&url).send().await?;

        if res.status().is_success() {
            let json: Value = res.json().await?;
            if let Some(beta_version) = json["dist-tags"]["beta"].as_str() {
                table.add_row(row![pkg, beta_version]);
                csv_data.push(format!("{},{}", pkg, beta_version));
                println!("✅ [{}/{}] Found beta for {}", i + 1, PACKAGES.len(), pkg);
            } else {
                println!("➖ [{}/{}] No beta tag for {}", i + 1, PACKAGES.len(), pkg);
            }
        } else {
            println!("❌ [{}/{}] Failed to fetch {}", i + 1, PACKAGES.len(), pkg);
        }

        sleep(Duration::from_millis(500)).await; // half-second delay between requests
    }

    table.printstd();

    let mut file = File::create("results.csv")?;
    for line in csv_data {
        writeln!(file, "{}", line)?;
    }

    println!("✅ Results saved to results.csv");
    Ok(())
}