// Generated macro for impl_305 (impl)
macro_rules! Depcrate_rngs_stdimpl_305 {
() => {
// Module: crate::rngs::std
// Provides: {"impl_305"}
// Dependencies: {}
impl SeedableRng for StdRng { type Seed = [u8 ; 32] ; # [inline (always)] fn from_seed (seed : Self :: Seed) -> Self { StdRng (Rng :: from_seed (seed)) } }
};
}
