// Generated macro for tf512_impl (function)
macro_rules! Depcrate_compressortf512_impl {
() => {
// Module: crate::compressor
// Provides: {"tf512_impl"}
// Dependencies: {}
# [inline (always)] unsafe fn tf512_impl (cv : & mut X4 , data : * const u8) { # [allow (clippy :: cast_ptr_alignment)] let data = data as * const __m128i ; let d0 = _mm_loadu_si128 (data) ; let d1 = _mm_loadu_si128 (data . offset (1)) ; let d2 = _mm_loadu_si128 (data . offset (2)) ; let d3 = _mm_loadu_si128 (data . offset (3)) ; let y = transpose_a (X4 (d0 , d1 , d2 , d3)) ; let x = (* cv , y) . map (| c , x | _mm_xor_si128 (c , x)) ; let p = transpose_b (X8 (x . 0 , x . 1 , x . 2 , x . 3 , y . 0 , y . 1 , y . 2 , y . 3)) ; let p = rounds_p_q (p) ; let p = transpose_b_inv (p) ; let x = X4 (_mm_xor_si128 (p . 0 , p . 4) , _mm_xor_si128 (p . 1 , p . 5) , _mm_xor_si128 (p . 2 , p . 6) , _mm_xor_si128 (p . 3 , p . 7) ,) ; * cv = * cv ^ x ; }
};
}
