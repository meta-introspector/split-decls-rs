// Generated macro for transpose_o_b (function)
macro_rules! Depcrate_compressortranspose_o_b {
() => {
// Module: crate::compressor
// Provides: {"transpose_o_b"}
// Dependencies: {}
# [doc = " Matrix Transpose Output Step 2"] # [doc = " input: one 512-bit state with two rows in one xmm"] # [doc = " output: one 512-bit state with one row in the low bits of one xmm"] # [inline (always)] unsafe fn transpose_o_b (i : X4) -> X8 { let t0 = _mm_cvtsi64_si128 (0) ; X8 (_mm_unpacklo_epi64 (i . 0 , t0) , _mm_unpackhi_epi64 (i . 0 , t0) , _mm_unpacklo_epi64 (i . 1 , t0) , _mm_unpackhi_epi64 (i . 1 , t0) , _mm_unpacklo_epi64 (i . 2 , t0) , _mm_unpackhi_epi64 (i . 2 , t0) , _mm_unpacklo_epi64 (i . 3 , t0) , _mm_unpackhi_epi64 (i . 3 , t0) ,) }
};
}
