// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl Generate for DeleteKeyAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { public_key : PublicKey :: generate (rng) , } } }
};
}
