// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl Generate for StakeAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { stake : u64 :: generate (rng) , public_key : PublicKey :: generate (rng) , } } }
};
}
