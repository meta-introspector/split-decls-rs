// Generated macro for mul2 (function)
macro_rules! Depcrate_compressormul2 {
() => {
// Module: crate::compressor
// Provides: {"mul2"}
// Dependencies: {}
# [inline (always)] fn mul2 (i : __m128i) -> __m128i { unsafe { let all_1b = _mm_set1_epi64x (0x1b1b_1b1b_1b1b_1b1b) ; let j = _mm_and_si128 (_mm_cmpgt_epi8 (_mm_cvtsi64_si128 (0) , i) , all_1b) ; let i = _mm_add_epi8 (i , i) ; _mm_xor_si128 (i , j) } }
};
}
