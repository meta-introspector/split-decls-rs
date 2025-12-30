// Generated macro for impl_189 (impl)
macro_rules! Depcrate_x86_64_sse2impl_189 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_189"}
// Dependencies: {}
impl < S4 , NI > Swap64 for u128x1_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn swap1 (self) -> Self { swapi ! (self , 1 , 0xaa) } # [inline (always)] fn swap2 (self) -> Self { swapi ! (self , 2 , 0xcc) } # [inline (always)] fn swap4 (self) -> Self { swapi ! (self , 4 , 0xf0) } # [inline (always)] fn swap8 (self) -> Self { u128x1_sse2 :: new (unsafe { _mm_or_si128 (_mm_slli_epi16 (self . x , 8) , _mm_srli_epi16 (self . x , 8)) }) } # [inline (always)] fn swap16 (self) -> Self { u128x1_sse2 :: new (swap16_s2 (self . x)) } # [inline (always)] fn swap32 (self) -> Self { u128x1_sse2 :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b1011_0001) }) } # [inline (always)] fn swap64 (self) -> Self { u128x1_sse2 :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0100_1110) }) } }
};
}
