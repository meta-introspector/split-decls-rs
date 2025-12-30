// Generated macro for impl_55 (impl)
macro_rules! Depcrate_xoroshiro128starstarimpl_55 {
() => {
// Module: crate::xoroshiro128starstar
// Provides: {"impl_55"}
// Dependencies: {}
impl SeedableRng for Xoroshiro128StarStar { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoroshiro128StarStar`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] fn from_seed (seed : [u8 ; 16]) -> Xoroshiro128StarStar { deal_with_zero_seed ! (seed , Self , 16) ; let mut s = [0 ; 2] ; read_u64_into (& seed , & mut s) ; Xoroshiro128StarStar { s0 : s [0] , s1 : s [1] } } # [doc = " Seed a `Xoroshiro128StarStar` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoroshiro128StarStar { from_splitmix ! (seed) } }
};
}
