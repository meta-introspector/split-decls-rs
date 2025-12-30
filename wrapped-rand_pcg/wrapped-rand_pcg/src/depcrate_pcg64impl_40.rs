// Generated macro for impl_40 (impl)
macro_rules! Depcrate_pcg64impl_40 {
() => {
// Module: crate::pcg64
// Provides: {"impl_40"}
// Dependencies: {}
impl SeedableRng for Lcg64Xsh32 { type Seed = [u8 ; 16] ; # [doc = " We use a single 127-bit seed to initialise the state and select a stream."] # [doc = " One `seed` bit (lowest bit of `seed[8]`) is ignored."] fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u64 = [0u64 ; 2] ; le :: read_u64_into (& seed , & mut seed_u64) ; Lcg64Xsh32 :: from_state_incr (seed_u64 [0] , seed_u64 [1] | 1) } }
};
}
