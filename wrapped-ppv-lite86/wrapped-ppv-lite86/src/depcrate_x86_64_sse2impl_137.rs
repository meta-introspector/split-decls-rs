// Generated macro for impl_137 (impl)
macro_rules! Depcrate_x86_64_sse2impl_137 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_137"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > RotateEachWord64 for u64x2_sse2 < S3 , S4 , NI > { # [inline (always)] fn rotate_each_word_right32 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b10110001) }) } }
};
}
