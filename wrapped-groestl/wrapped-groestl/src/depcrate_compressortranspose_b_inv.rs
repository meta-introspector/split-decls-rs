// Generated macro for transpose_b_inv (function)
macro_rules! Depcrate_compressortranspose_b_inv {
() => {
// Module: crate::compressor
// Provides: {"transpose_b_inv"}
// Dependencies: {}
# [doc = " Matrix Transpose Inverse Step 2"] # [doc = " input: two 512-bit states with one row of each state in one xmm"] # [doc = " output: two 512-bit states with two rows in one xmm"] # [inline (always)] unsafe fn transpose_b_inv (i : X8) -> X8 { X8 (_mm_unpacklo_epi64 (i . 0 , i . 1) , _mm_unpacklo_epi64 (i . 2 , i . 3) , _mm_unpacklo_epi64 (i . 4 , i . 5) , _mm_unpacklo_epi64 (i . 6 , i . 7) , _mm_unpackhi_epi64 (i . 0 , i . 1) , _mm_unpackhi_epi64 (i . 2 , i . 3) , _mm_unpackhi_epi64 (i . 4 , i . 5) , _mm_unpackhi_epi64 (i . 6 , i . 7) ,) }
};
}
