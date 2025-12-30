// Generated macro for impl_97 (impl)
macro_rules! Depcrate_xoshiro128starstarimpl_97 {
() => {
// Module: crate::xoshiro128starstar
// Provides: {"impl_97"}
// Dependencies: {}
impl SeedableRng for Xoshiro128StarStar { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoshiro128StarStar`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 16]) -> Xoshiro128StarStar { deal_with_zero_seed ! (seed , Self , 16) ; let mut state = [0 ; 4] ; read_u32_into (& seed , & mut state) ; Xoshiro128StarStar { s : state } } # [doc = " Seed a `Xoshiro128StarStar` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro128StarStar { from_splitmix ! (seed) } }
};
}
