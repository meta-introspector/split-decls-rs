// Generated macro for solana_signer (module)
macro_rules! Depcratesolana_signer {
() => {
// Module: crate
// Provides: {"solana_signer"}
// Dependencies: {}
pub mod solana_signer { use { solana_pubkey :: Pubkey , thiserror :: Error } ; # [derive (Error , Debug)] # [error ("mock-error")] pub struct SignerError ; pub trait Signer { fn pubkey (& self) -> Pubkey ; } pub mod signers { use super :: Signer ; pub trait Signers { } impl < T : Signer > Signers for [& T] { } impl < T : Signer > Signers for [& T ; 1] { } impl < T : Signer > Signers for [& T ; 2] { } } }
};
}
