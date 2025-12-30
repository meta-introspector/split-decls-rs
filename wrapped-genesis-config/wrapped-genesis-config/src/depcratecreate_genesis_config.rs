// Generated macro for create_genesis_config (function)
macro_rules! Depcratecreate_genesis_config {
() => {
// Module: crate
// Provides: {"create_genesis_config"}
// Dependencies: {}
pub fn create_genesis_config (lamports : u64) -> (GenesisConfig , Keypair) { let faucet_keypair = Keypair :: new () ; (GenesisConfig :: new (& [(faucet_keypair . pubkey () , AccountSharedData :: new (lamports , 0 , & system_program :: id ()) ,)] , & [] ,) , faucet_keypair ,) }
};
}
