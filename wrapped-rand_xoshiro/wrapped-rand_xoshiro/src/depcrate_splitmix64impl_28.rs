// Generated macro for impl_28 (impl)
macro_rules! Depcrate_splitmix64impl_28 {
() => {
// Module: crate::splitmix64
// Provides: {"impl_28"}
// Dependencies: {}
impl SeedableRng for SplitMix64 { type Seed = [u8 ; 8] ; # [doc = " Create a new `SplitMix64`."] fn from_seed (seed : [u8 ; 8]) -> SplitMix64 { let mut state = [0 ; 1] ; read_u64_into (& seed , & mut state) ; SplitMix64 { x : state [0] } } # [doc = " Seed a `SplitMix64` from a `u64`."] fn seed_from_u64 (seed : u64) -> SplitMix64 { SplitMix64 :: from_seed (seed . to_le_bytes ()) } }
};
}
