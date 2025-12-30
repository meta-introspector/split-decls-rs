// Generated macro for impl_173 (impl)
macro_rules! Depcrate_x86_64_sse2impl_173 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_173"}
// Dependencies: {}
impl < S3 , S4 , NI > Words4 for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn shuffle2301 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0100_1110) }) } # [inline (always)] fn shuffle1230 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b1001_0011) }) } # [inline (always)] fn shuffle3012 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0011_1001) }) } }
};
}
