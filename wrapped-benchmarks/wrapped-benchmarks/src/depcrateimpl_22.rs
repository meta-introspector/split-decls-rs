// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Generate for (PublicKey , PublicKey) { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { (PublicKey :: generate (rng) , PublicKey :: generate (rng)) } }
};
}
