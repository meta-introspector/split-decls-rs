// Generated macro for impl_23 (impl)
macro_rules! Depcrate_murmur3impl_23 {
() => {
// Module: crate::murmur3
// Provides: {"impl_23"}
// Dependencies: {}
impl crate :: Hasher for Murmur3Hasher { fn finish32 (& self) -> u32 { let mut state = match self . index { Index :: _3 => { let mut block = 0 ; unsafe { block ^= u32 :: from (self . buf . bytes . assume_init_ref () [2]) << 16 ; block ^= u32 :: from (self . buf . bytes . assume_init_ref () [1]) << 8 ; block ^= u32 :: from (self . buf . bytes . assume_init_ref () [0]) ; } self . state . 0 ^ pre_mix (block) } Index :: _2 => { let mut block = 0 ; unsafe { block ^= u32 :: from (self . buf . bytes . assume_init_ref () [1]) << 8 ; block ^= u32 :: from (self . buf . bytes . assume_init_ref () [0]) ; } self . state . 0 ^ pre_mix (block) } Index :: _1 => { let mut block = 0 ; unsafe { block ^= u32 :: from (self . buf . bytes . assume_init_ref () [0]) ; } self . state . 0 ^ pre_mix (block) } Index :: _0 => self . state . 0 , } ; state ^= self . processed ; state ^= state >> 16 ; state = state . wrapping_mul (0x85ebca6b) ; state ^= state >> 13 ; state = state . wrapping_mul (0xc2b2ae35) ; state ^= state >> 16 ; state } }
};
}
