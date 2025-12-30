// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl Generate for AddKeyAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { public_key : PublicKey :: generate (rng) , access_key : AccessKey :: generate (rng) , } } }
};
}
