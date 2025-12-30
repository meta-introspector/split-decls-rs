// Generated macro for impl_37 (impl)
macro_rules! Depcrate_xoroshiro128plusimpl_37 {
() => {
// Module: crate::xoroshiro128plus
// Provides: {"impl_37"}
// Dependencies: {}
impl SeedableRng for Xoroshiro128Plus { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoroshiro128Plus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] fn from_seed (seed : [u8 ; 16]) -> Xoroshiro128Plus { deal_with_zero_seed ! (seed , Self , 16) ; let mut s = [0 ; 2] ; read_u64_into (& seed , & mut s) ; Xoroshiro128Plus { s0 : s [0] , s1 : s [1] } } # [doc = " Seed a `Xoroshiro128Plus` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoroshiro128Plus { from_splitmix ! (seed) } }
};
}
