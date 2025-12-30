// Generated macro for transpose_a (function)
macro_rules! Depcrate_compressortranspose_a {
() => {
// Module: crate::compressor
// Provides: {"transpose_a"}
// Dependencies: {}
# [doc = " Matrix Transpose Step 1"] # [doc = " input: a 512-bit state with two columns in one xmm"] # [doc = " output: a 512-bit state with two rows in one xmm"] # [inline (always)] unsafe fn transpose_a (i : X4) -> X4 { let mask = _mm_set_epi64x (0x0f07_0b03_0e06_0a02 , 0x0d05_0901_0c04_0800) ; let i = i . map (| x | _mm_shuffle_epi8 (x , mask)) ; let z = X4 (_mm_unpacklo_epi16 (i . 0 , i . 1) , _mm_unpackhi_epi16 (i . 0 , i . 1) , _mm_unpacklo_epi16 (i . 2 , i . 3) , _mm_unpackhi_epi16 (i . 2 , i . 3) ,) . map (| x | _mm_shuffle_epi32 (x , 0b1101_1000)) ; X4 (_mm_unpacklo_epi32 (z . 0 , z . 2) , _mm_unpacklo_epi32 (z . 1 , z . 3) , _mm_unpackhi_epi32 (z . 0 , z . 2) , _mm_unpackhi_epi32 (z . 1 , z . 3) ,) }
};
}
