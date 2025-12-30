// Generated macro for impl_88 (impl)
macro_rules! Depcrate_xoshiro128plusplusimpl_88 {
() => {
// Module: crate::xoshiro128plusplus
// Provides: {"impl_88"}
// Dependencies: {}
impl SeedableRng for Xoshiro128PlusPlus { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoshiro128PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 16]) -> Xoshiro128PlusPlus { deal_with_zero_seed ! (seed , Self , 16) ; let mut state = [0 ; 4] ; read_u32_into (& seed , & mut state) ; Xoshiro128PlusPlus { s : state } } # [doc = " Seed a `Xoshiro128PlusPlus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro128PlusPlus { from_splitmix ! (seed) } }
};
}
