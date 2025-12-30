// Generated macro for impl_134 (impl)
macro_rules! Depcrate_xoshiro512plusimpl_134 {
() => {
// Module: crate::xoshiro512plus
// Provides: {"impl_134"}
// Dependencies: {}
impl SeedableRng for Xoshiro512Plus { type Seed = Seed512 ; # [doc = " Create a new `Xoshiro512Plus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : Seed512) -> Xoshiro512Plus { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 8] ; read_u64_into (& seed . 0 , & mut state) ; Xoshiro512Plus { s : state } } # [doc = " Seed a `Xoshiro512Plus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro512Plus { from_splitmix ! (seed) } }
};
}
