// Generated macro for impl_63 (impl)
macro_rules! Depcrate_xoroshiro64starimpl_63 {
() => {
// Module: crate::xoroshiro64star
// Provides: {"impl_63"}
// Dependencies: {}
impl SeedableRng for Xoroshiro64Star { type Seed = [u8 ; 8] ; # [doc = " Create a new `Xoroshiro64Star`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] fn from_seed (seed : [u8 ; 8]) -> Xoroshiro64Star { deal_with_zero_seed ! (seed , Self , 8) ; let mut s = [0 ; 2] ; read_u32_into (& seed , & mut s) ; Xoroshiro64Star { s0 : s [0] , s1 : s [1] } } # [doc = " Seed a `Xoroshiro64Star` from a `u64` using `SplitMix64`."] fn seed_from_u64 (seed : u64) -> Xoroshiro64Star { from_splitmix ! (seed) } }
};
}
