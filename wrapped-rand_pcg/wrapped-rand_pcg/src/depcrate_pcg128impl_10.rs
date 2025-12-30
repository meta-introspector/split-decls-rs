// Generated macro for impl_10 (impl)
macro_rules! Depcrate_pcg128impl_10 {
() => {
// Module: crate::pcg128
// Provides: {"impl_10"}
// Dependencies: {}
impl SeedableRng for Lcg128Xsl64 { type Seed = [u8 ; 32] ; # [doc = " We use a single 255-bit seed to initialise the state and select a stream."] # [doc = " One `seed` bit (lowest bit of `seed[8]`) is ignored."] fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u64 = [0u64 ; 4] ; le :: read_u64_into (& seed , & mut seed_u64) ; let state = u128 :: from (seed_u64 [0]) | (u128 :: from (seed_u64 [1]) << 64) ; let incr = u128 :: from (seed_u64 [2]) | (u128 :: from (seed_u64 [3]) << 64) ; Lcg128Xsl64 :: from_state_incr (state , incr | 1) } }
};
}
