// Generated macro for of512_impl (function)
macro_rules! Depcrate_compressorof512_impl {
() => {
// Module: crate::compressor
// Provides: {"of512_impl"}
// Dependencies: {}
# [inline (always)] unsafe fn of512_impl (cv : & mut X4) { let p = transpose_o_b (* cv) ; let p = rounds_p_q (p) ; let p = * cv ^ transpose_o_b_inv (p) ; let X4 (_ , _ , x9 , x11) = transpose_a (p) ; cv . 2 = x9 ; cv . 3 = x11 ; }
};
}
