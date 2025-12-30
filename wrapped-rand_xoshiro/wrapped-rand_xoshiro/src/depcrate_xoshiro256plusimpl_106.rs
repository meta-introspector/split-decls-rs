// Generated macro for impl_106 (impl)
macro_rules! Depcrate_xoshiro256plusimpl_106 {
() => {
// Module: crate::xoshiro256plus
// Provides: {"impl_106"}
// Dependencies: {}
impl SeedableRng for Xoshiro256Plus { type Seed = [u8 ; 32] ; # [doc = " Create a new `Xoshiro256Plus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 32]) -> Xoshiro256Plus { deal_with_zero_seed ! (seed , Self) ; let mut state = [0 ; 4] ; read_u64_into (& seed , & mut state) ; Xoshiro256Plus { s : state } } # [doc = " Seed a `Xoshiro256Plus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro256Plus { from_splitmix ! (seed) } }
};
}
