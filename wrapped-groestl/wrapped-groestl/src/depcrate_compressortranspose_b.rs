// Generated macro for transpose_b (function)
macro_rules! Depcrate_compressortranspose_b {
() => {
// Module: crate::compressor
// Provides: {"transpose_b"}
// Dependencies: {}
# [doc = " Matrix Transpose Step 2"] # [doc = " input: two 512-bit states with two rows in one xmm"] # [doc = " output: two 512-bit states with one row of each state in one xmm"] # [inline (always)] unsafe fn transpose_b (i : X8) -> X8 { X8 (_mm_unpacklo_epi64 (i . 0 , i . 4) , _mm_unpackhi_epi64 (i . 0 , i . 4) , _mm_unpacklo_epi64 (i . 1 , i . 5) , _mm_unpackhi_epi64 (i . 1 , i . 5) , _mm_unpacklo_epi64 (i . 2 , i . 6) , _mm_unpackhi_epi64 (i . 2 , i . 6) , _mm_unpacklo_epi64 (i . 3 , i . 7) , _mm_unpackhi_epi64 (i . 3 , i . 7) ,) }
};
}
