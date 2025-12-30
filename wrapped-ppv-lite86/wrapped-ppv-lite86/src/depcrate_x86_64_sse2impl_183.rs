// Generated macro for impl_183 (impl)
macro_rules! Depcrate_x86_64_sse2impl_183 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_183"}
// Dependencies: {}
impl < S4 , NI > BSwap for u64x2_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { bswap32_s2 (_mm_shuffle_epi32 (self . x , 0b1011_0001)) }) } }
};
}
