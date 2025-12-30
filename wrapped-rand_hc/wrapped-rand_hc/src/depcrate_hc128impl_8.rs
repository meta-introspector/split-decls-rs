// Generated macro for impl_8 (impl)
macro_rules! Depcrate_hc128impl_8 {
() => {
// Module: crate::hc128
// Provides: {"impl_8"}
// Dependencies: {}
impl SeedableRng for Hc128Rng { type Seed = < Hc128Core as SeedableRng > :: Seed ; # [inline] fn from_seed (seed : Self :: Seed) -> Self { Hc128Rng (BlockRng :: < Hc128Core > :: from_seed (seed)) } }
};
}
