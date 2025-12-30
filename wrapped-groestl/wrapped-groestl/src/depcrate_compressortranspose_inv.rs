// Generated macro for transpose_inv (function)
macro_rules! Depcrate_compressortranspose_inv {
() => {
// Module: crate::compressor
// Provides: {"transpose_inv"}
// Dependencies: {}
# [doc = " transpose matrix to get output format"] # [inline (always)] unsafe fn transpose_inv (i : X8) -> X8 { let i = X8 (_mm_unpacklo_epi64 (i . 0 , i . 1) , _mm_unpackhi_epi64 (i . 0 , i . 1) , _mm_unpacklo_epi64 (i . 2 , i . 3) , _mm_unpackhi_epi64 (i . 2 , i . 3) , _mm_unpacklo_epi64 (i . 4 , i . 5) , _mm_unpackhi_epi64 (i . 4 , i . 5) , _mm_unpacklo_epi64 (i . 6 , i . 7) , _mm_unpackhi_epi64 (i . 6 , i . 7) ,) . map (| x | { _mm_shuffle_epi8 (x , _mm_set_epi64x (0x0f07_0b03_0e06_0a02 , 0x0d05_0901_0c04_0800) ,) }) ; let i = X8 (_mm_unpacklo_epi16 (i . 0 , i . 2) , _mm_unpacklo_epi16 (i . 1 , i . 3) , _mm_unpackhi_epi16 (i . 0 , i . 2) , _mm_unpackhi_epi16 (i . 1 , i . 3) , _mm_unpacklo_epi16 (i . 4 , i . 6) , _mm_unpacklo_epi16 (i . 5 , i . 7) , _mm_unpackhi_epi16 (i . 4 , i . 6) , _mm_unpackhi_epi16 (i . 5 , i . 7) ,) . map (| x | _mm_shuffle_epi32 (x , 0b1101_1000)) ; X8 (_mm_unpacklo_epi32 (i . 0 , i . 4) , _mm_unpacklo_epi32 (i . 2 , i . 6) , _mm_unpackhi_epi32 (i . 0 , i . 4) , _mm_unpackhi_epi32 (i . 2 , i . 6) , _mm_unpacklo_epi32 (i . 1 , i . 5) , _mm_unpacklo_epi32 (i . 3 , i . 7) , _mm_unpackhi_epi32 (i . 1 , i . 5) , _mm_unpackhi_epi32 (i . 3 , i . 7) ,) }
};
}
