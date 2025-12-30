// Generated macro for impl_115 (impl)
macro_rules! Depcrate_xoshiro256plusplusimpl_115 {
() => {
// Module: crate::xoshiro256plusplus
// Provides: {"impl_115"}
// Dependencies: {}
impl SeedableRng for Xoshiro256PlusPlus { type Seed = [u8 ; 32] ; # [doc = " Create a new `Xoshiro256PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 32]) -> Xoshiro256PlusPlus { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 4] ; read_u64_into (& seed , & mut state) ; Xoshiro256PlusPlus { s : state } } # [doc = " Seed a `Xoshiro256PlusPlus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro256PlusPlus { from_splitmix ! (seed) } }
};
}
