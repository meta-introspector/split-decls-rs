// Generated macro for transpose (function)
macro_rules! Depcrate_compressortranspose {
() => {
// Module: crate::compressor
// Provides: {"transpose"}
// Dependencies: {}
# [inline (always)] unsafe fn transpose (i : X8) -> X8 { let i = i . map (| x | { _mm_shuffle_epi8 (x , _mm_set_epi64x (0x0f07_0b03_0e06_0a02 , 0x0d05_0901_0c04_0800) ,) }) ; let (eve , odd) = (X4 (i . 0 , i . 2 , i . 4 , i . 6) , X4 (i . 1 , i . 3 , i . 5 , i . 7)) ; let i = (eve , odd) . map (| e , o | _mm_shuffle_epi32 (_mm_unpacklo_epi16 (e , o) , 0b1101_1000)) ; let t = (eve , odd) . map (| e , o | _mm_shuffle_epi32 (_mm_unpackhi_epi16 (e , o) , 0b1101_1000)) ; let t = X8 (_mm_unpacklo_epi32 (t . 0 , t . 1) , _mm_unpacklo_epi32 (i . 0 , i . 1) , _mm_unpacklo_epi32 (t . 2 , t . 3) , _mm_unpacklo_epi32 (i . 2 , i . 3) , _mm_unpackhi_epi32 (i . 0 , i . 1) , _mm_unpackhi_epi32 (t . 0 , t . 1) , _mm_unpackhi_epi32 (i . 2 , i . 3) , _mm_unpackhi_epi32 (t . 2 , t . 3) ,) ; X8 (_mm_unpacklo_epi64 (t . 1 , t . 3) , _mm_unpackhi_epi64 (t . 1 , t . 3) , _mm_unpacklo_epi64 (t . 0 , t . 2) , _mm_unpackhi_epi64 (t . 0 , t . 2) , _mm_unpacklo_epi64 (t . 4 , t . 6) , _mm_unpackhi_epi64 (t . 4 , t . 6) , _mm_unpacklo_epi64 (t . 5 , t . 7) , _mm_unpackhi_epi64 (t . 5 , t . 7) ,) }
};
}
