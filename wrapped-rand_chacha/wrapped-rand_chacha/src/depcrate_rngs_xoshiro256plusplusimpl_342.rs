// Generated macro for impl_342 (impl)
macro_rules! Depcrate_rngs_xoshiro256plusplusimpl_342 {
() => {
// Module: crate::rngs::xoshiro256plusplus
// Provides: {"impl_342"}
// Dependencies: {}
impl SeedableRng for Xoshiro256PlusPlus { type Seed = [u8 ; 32] ; # [doc = " Create a new `Xoshiro256PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 32]) -> Xoshiro256PlusPlus { let mut state = [0 ; 4] ; le :: read_u64_into (& seed , & mut state) ; if state . iter () . all (| & x | x == 0) { return Self :: seed_from_u64 (0) ; } Xoshiro256PlusPlus { s : state } } # [doc = " Create a new `Xoshiro256PlusPlus` from a `u64` seed."] # [doc = ""] # [doc = " This uses the SplitMix64 generator internally."] # [inline] fn seed_from_u64 (mut state : u64) -> Self { const PHI : u64 = 0x9e3779b97f4a7c15 ; let mut s = [0 ; 4] ; for i in s . iter_mut () { state = state . wrapping_add (PHI) ; let mut z = state ; z = (z ^ (z >> 30)) . wrapping_mul (0xbf58476d1ce4e5b9) ; z = (z ^ (z >> 27)) . wrapping_mul (0x94d049bb133111eb) ; z = z ^ (z >> 31) ; * i = z ; } debug_assert_ne ! (s , [0 ; 4]) ; Xoshiro256PlusPlus { s } } }
};
}
