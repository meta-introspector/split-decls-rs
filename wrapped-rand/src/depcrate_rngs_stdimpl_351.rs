// Generated macro for impl_351 (impl)
macro_rules! Depcrate_rngs_stdimpl_351 {
() => {
// Module: crate::rngs::std
// Provides: {"impl_351"}
// Dependencies: {}
impl SeedableRng for StdRng { type Seed = [u8 ; 32] ; # [inline (always)] fn from_seed (seed : Self :: Seed) -> Self { StdRng (Rng :: from_seed (seed)) } }
};
}
