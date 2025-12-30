// Generated macro for solana_keypair (module)
macro_rules! Depcratesolana_keypair {
() => {
// Module: crate
// Provides: {"solana_keypair"}
// Dependencies: {}
pub mod solana_keypair { use { crate :: solana_signer :: Signer , solana_pubkey :: Pubkey } ; pub struct Keypair ; impl Keypair { pub fn new () -> Keypair { Keypair } } impl Signer for Keypair { fn pubkey (& self) -> Pubkey { Pubkey :: default () } } }
};
}
