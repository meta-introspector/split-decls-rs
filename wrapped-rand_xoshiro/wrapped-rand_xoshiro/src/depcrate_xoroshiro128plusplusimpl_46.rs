// Generated macro for impl_46 (impl)
macro_rules! Depcrate_xoroshiro128plusplusimpl_46 {
() => {
// Module: crate::xoroshiro128plusplus
// Provides: {"impl_46"}
// Dependencies: {}
impl SeedableRng for Xoroshiro128PlusPlus { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoroshiro128PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] fn from_seed (seed : [u8 ; 16]) -> Xoroshiro128PlusPlus { deal_with_zero_seed ! (seed , Self , 16) ; let mut s = [0 ; 2] ; read_u64_into (& seed , & mut s) ; Xoroshiro128PlusPlus { s0 : s [0] , s1 : s [1] } } # [doc = " Seed a `Xoroshiro128PlusPlus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoroshiro128PlusPlus { from_splitmix ! (seed) } }
};
}
