// Generated macro for impl_181 (impl)
macro_rules! Depcrate_x86_64_sse2impl_181 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_181"}
// Dependencies: {}
impl < S4 , NI > BSwap for u32x4_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (bswap32_s2 (self . x)) } }
};
}
