// Generated macro for impl_16 (impl)
macro_rules! Depcrate_pcg128impl_16 {
() => {
// Module: crate::pcg128
// Provides: {"impl_16"}
// Dependencies: {}
# [doc = " We use a single 126-bit seed to initialise the state and select a stream."] # [doc = " Two `seed` bits (lowest order of last byte) are ignored."] impl SeedableRng for Mcg128Xsl64 { type Seed = [u8 ; 16] ; fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u64 = [0u64 ; 2] ; le :: read_u64_into (& seed , & mut seed_u64) ; let state = u128 :: from (seed_u64 [0]) | (u128 :: from (seed_u64 [1]) << 64) ; Mcg128Xsl64 :: new (state) } }
};
}
