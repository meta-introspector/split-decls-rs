// Generated macro for impl_179 (impl)
macro_rules! Depcrate_x86_64_sse2impl_179 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_179"}
// Dependencies: {}
impl < S4 , NI > BSwap for u32x4_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0c0d_0e0f_0809_0a0b , 0x0405_0607_0001_0203) ; _mm_shuffle_epi8 (self . x , k) }) } }
};
}
