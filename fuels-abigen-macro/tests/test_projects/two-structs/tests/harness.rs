use fuel_core::service::{Config, FuelService};
use fuel_tx::Salt;
use fuels_abigen_macro::abigen;
use fuels_rs::contract::Contract;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

abigen!(MyContract, "./abi.json");

#[tokio::test]
async fn harness() {
    let rng = &mut StdRng::seed_from_u64(2322u64);

    // Build the contract
    let salt: [u8; 32] = rng.gen();
    let salt = Salt::from(salt);
    let compiled = Contract::compile_sway_contract("./", salt).unwrap();

    // Launch a local network and deploy the contract
    let client = Provider::launch(Config::local_node()).await.unwrap();
    let contract_id = Contract::deployed(&compiled, &client).await.unwrap();

    let contract_instance = MyContract::new(contract_id.to_string(), client);
}
