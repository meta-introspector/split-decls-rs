// Generated macro for impl_144 (impl)
macro_rules! Depcrate_xoshiro512plusplusimpl_144 {
() => {
// Module: crate::xoshiro512plusplus
// Provides: {"impl_144"}
// Dependencies: {}
impl SeedableRng for Xoshiro512PlusPlus { type Seed = Seed512 ; # [doc = " Create a new `Xoshiro512PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : Seed512) -> Xoshiro512PlusPlus { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 8] ; read_u64_into (& seed . 0 , & mut state) ; Xoshiro512PlusPlus { s : state } } # [doc = " Seed a `Xoshiro512PlusPlus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro512PlusPlus { from_splitmix ! (seed) } }
};
}
