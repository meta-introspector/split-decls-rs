// Generated macro for tf1024_impl (function)
macro_rules! Depcrate_compressortf1024_impl {
() => {
// Module: crate::compressor
// Provides: {"tf1024_impl"}
// Dependencies: {}
# [inline (always)] unsafe fn tf1024_impl (cv : & mut X8 , data : * const u8) { # [allow (clippy :: cast_ptr_alignment)] let data = data as * const __m128i ; let p = X8 (_mm_loadu_si128 (data) , _mm_loadu_si128 (data . offset (1)) , _mm_loadu_si128 (data . offset (2)) , _mm_loadu_si128 (data . offset (3)) , _mm_loadu_si128 (data . offset (4)) , _mm_loadu_si128 (data . offset (5)) , _mm_loadu_si128 (data . offset (6)) , _mm_loadu_si128 (data . offset (7)) ,) ; let q = transpose (p) ; * cv = * cv ^ rounds_p (* cv ^ q) ; * cv = * cv ^ rounds_q (q) ; }
};
}
