// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl Generate for SignedTransaction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { transaction : Transaction :: generate (rng) , signature : Signature :: generate (rng) , hash : CryptoHash :: generate (rng) , } } }
};
}
