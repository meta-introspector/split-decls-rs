// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl Generate for BlockHeader { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { inner : BlockHeaderInner :: generate (rng) , signature : Signature :: generate (rng) , hash : CryptoHash :: generate (rng) , } } }
};
}
