// Generated macro for eq128_s4 (function)
macro_rules! Depcrate_x86_64_sse2eq128_s4 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"eq128_s4"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] unsafe fn eq128_s4 (x : __m128i , y : __m128i) -> bool { let q = _mm_shuffle_epi32 (_mm_cmpeq_epi64 (x , y) , 0b1100_0110) ; _mm_cvtsi128_si64 (q) == - 1 }
};
}
