// Generated macro for transpose_o_b_inv (function)
macro_rules! Depcrate_compressortranspose_o_b_inv {
() => {
// Module: crate::compressor
// Provides: {"transpose_o_b_inv"}
// Dependencies: {}
# [doc = " Matrix Transpose Output Inverse Step 2"] # [doc = " input: one 512-bit state with one row in the low bits of one xmm"] # [doc = " output: one 512-bit state with two rows in one xmm"] # [inline (always)] unsafe fn transpose_o_b_inv (i : X8) -> X4 { (X4 (i . 0 , i . 2 , i . 4 , i . 6) , X4 (i . 1 , i . 3 , i . 5 , i . 7)) . map (| e , o | _mm_unpacklo_epi64 (e , o)) }
};
}
