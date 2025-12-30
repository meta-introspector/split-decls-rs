// Generated macro for impl_182 (impl)
macro_rules! Depcrate_x86_64_sse2impl_182 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_182"}
// Dependencies: {}
impl < S4 , NI > BSwap for u64x2_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0809_0a0b_0c0d_0e0f , 0x0001_0203_0405_0607) ; _mm_shuffle_epi8 (self . x , k) }) } }
};
}
