// Generated macro for match_header_value_char_16_sse (function)
macro_rules! Depcrate_simd_sse42match_header_value_char_16_sse {
() => {
// Module: crate::simd::sse42
// Provides: {"match_header_value_char_16_sse"}
// Dependencies: {}
# [inline (always)] # [allow (non_snake_case)] unsafe fn match_header_value_char_16_sse (buf : & [u8]) -> usize { debug_assert ! (buf . len () >= 16) ; # [cfg (target_arch = "x86")] use core :: arch :: x86 :: * ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: * ; let ptr = buf . as_ptr () ; let TAB : __m128i = _mm_set1_epi8 (0x09) ; let DEL : __m128i = _mm_set1_epi8 (0x7f) ; let LOW : __m128i = _mm_set1_epi8 (0x20) ; let dat = _mm_lddqu_si128 (ptr as * const _) ; let low = _mm_cmpeq_epi8 (_mm_max_epu8 (dat , LOW) , dat) ; let tab = _mm_cmpeq_epi8 (dat , TAB) ; let del = _mm_cmpeq_epi8 (dat , DEL) ; let bit = _mm_andnot_si128 (del , _mm_or_si128 (low , tab)) ; let res = _mm_movemask_epi8 (bit) as u16 ; res . trailing_ones () as usize }
};
}
