// Generated macro for impl_184 (impl)
macro_rules! Depcrate_x86_64_sse2impl_184 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_184"}
// Dependencies: {}
impl < S4 , NI > BSwap for u128x1_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0f0e_0d0c_0b0a_0908 , 0x0706_0504_0302_0100) ; _mm_shuffle_epi8 (self . x , k) }) } }
};
}
