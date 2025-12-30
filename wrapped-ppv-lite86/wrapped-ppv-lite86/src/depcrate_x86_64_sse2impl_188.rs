// Generated macro for impl_188 (impl)
macro_rules! Depcrate_x86_64_sse2impl_188 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_188"}
// Dependencies: {}
impl < S4 , NI > Swap64 for u128x1_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn swap1 (self) -> Self { swapi ! (self , 1 , 0xaa) } # [inline (always)] fn swap2 (self) -> Self { swapi ! (self , 2 , 0xcc) } # [inline (always)] fn swap4 (self) -> Self { swapi ! (self , 4 , 0xf0) } # [inline (always)] fn swap8 (self) -> Self { u128x1_sse2 :: new (unsafe { let k = _mm_set_epi64x (0x0e0f_0c0d_0a0b_0809 , 0x0607_0405_0203_0001) ; _mm_shuffle_epi8 (self . x , k) }) } # [inline (always)] fn swap16 (self) -> Self { u128x1_sse2 :: new (unsafe { let k = _mm_set_epi64x (0x0d0c_0f0e_0908_0b0a , 0x0504_0706_0100_0302) ; _mm_shuffle_epi8 (self . x , k) }) } # [inline (always)] fn swap32 (self) -> Self { u128x1_sse2 :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b1011_0001) }) } # [inline (always)] fn swap64 (self) -> Self { u128x1_sse2 :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0100_1110) }) } }
};
}
