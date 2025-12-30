// Generated macro for impl_71 (impl)
macro_rules! Depcrate_xoroshiro64starstarimpl_71 {
() => {
// Module: crate::xoroshiro64starstar
// Provides: {"impl_71"}
// Dependencies: {}
impl SeedableRng for Xoroshiro64StarStar { type Seed = [u8 ; 8] ; # [doc = " Create a new `Xoroshiro64StarStar`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] fn from_seed (seed : [u8 ; 8]) -> Xoroshiro64StarStar { deal_with_zero_seed ! (seed , Self , 8) ; let mut s = [0 ; 2] ; read_u32_into (& seed , & mut s) ; Xoroshiro64StarStar { s0 : s [0] , s1 : s [1] } } # [doc = " Seed a `Xoroshiro64StarStar` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoroshiro64StarStar { from_splitmix ! (seed) } }
};
}
