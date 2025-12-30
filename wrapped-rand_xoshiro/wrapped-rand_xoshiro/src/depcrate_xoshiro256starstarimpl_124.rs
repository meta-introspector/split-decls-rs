// Generated macro for impl_124 (impl)
macro_rules! Depcrate_xoshiro256starstarimpl_124 {
() => {
// Module: crate::xoshiro256starstar
// Provides: {"impl_124"}
// Dependencies: {}
impl SeedableRng for Xoshiro256StarStar { type Seed = [u8 ; 32] ; # [doc = " Create a new `Xoshiro256StarStar`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 32]) -> Xoshiro256StarStar { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 4] ; read_u64_into (& seed , & mut state) ; Xoshiro256StarStar { s : state } } # [doc = " Seed a `Xoshiro256StarStar` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro256StarStar { from_splitmix ! (seed) } }
};
}
