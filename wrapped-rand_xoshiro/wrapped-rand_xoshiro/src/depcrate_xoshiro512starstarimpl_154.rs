// Generated macro for impl_154 (impl)
macro_rules! Depcrate_xoshiro512starstarimpl_154 {
() => {
// Module: crate::xoshiro512starstar
// Provides: {"impl_154"}
// Dependencies: {}
impl SeedableRng for Xoshiro512StarStar { type Seed = Seed512 ; # [doc = " Create a new `Xoshiro512StarStar`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : Seed512) -> Xoshiro512StarStar { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 8] ; read_u64_into (& seed . 0 , & mut state) ; Xoshiro512StarStar { s : state } } # [doc = " Seed a `Xoshiro512StarStar` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro512StarStar { from_splitmix ! (seed) } }
};
}
