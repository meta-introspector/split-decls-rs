// Generated macro for match_header_value_char_32_avx (function)
macro_rules! Depcrate_simd_avx2match_header_value_char_32_avx {
() => {
// Module: crate::simd::avx2
// Provides: {"match_header_value_char_32_avx"}
// Dependencies: {}
# [inline (always)] # [allow (non_snake_case)] # [allow (unused)] unsafe fn match_header_value_char_32_avx (buf : & [u8]) -> usize { debug_assert ! (buf . len () >= 32) ; # [cfg (target_arch = "x86")] use core :: arch :: x86 :: * ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: * ; let ptr = buf . as_ptr () ; let TAB : __m256i = _mm256_set1_epi8 (0x09) ; let DEL : __m256i = _mm256_set1_epi8 (0x7f) ; let LOW : __m256i = _mm256_set1_epi8 (0x20) ; let dat = _mm256_lddqu_si256 (ptr as * const _) ; let low = _mm256_cmpeq_epi8 (_mm256_max_epu8 (dat , LOW) , dat) ; let tab = _mm256_cmpeq_epi8 (dat , TAB) ; let del = _mm256_cmpeq_epi8 (dat , DEL) ; let bit = _mm256_andnot_si256 (del , _mm256_or_si256 (low , tab)) ; let res = _mm256_movemask_epi8 (bit) as u32 ; res . trailing_ones () as usize }
};
}
