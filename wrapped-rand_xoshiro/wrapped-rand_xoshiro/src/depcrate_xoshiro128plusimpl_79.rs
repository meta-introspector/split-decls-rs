// Generated macro for impl_79 (impl)
macro_rules! Depcrate_xoshiro128plusimpl_79 {
() => {
// Module: crate::xoshiro128plus
// Provides: {"impl_79"}
// Dependencies: {}
impl SeedableRng for Xoshiro128Plus { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoshiro128Plus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 16]) -> Xoshiro128Plus { deal_with_zero_seed ! (seed , Self , 16) ; let mut state = [0 ; 4] ; read_u32_into (& seed , & mut state) ; Xoshiro128Plus { s : state } } # [doc = " Seed a `Xoshiro128Plus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoshiro128Plus { from_splitmix ! (seed) } }
};
}
