// Generated macro for rounds_p (function)
macro_rules! Depcrate_compressorrounds_p {
() => {
// Module: crate::compressor
// Provides: {"rounds_p"}
// Dependencies: {}
# [inline (always)] unsafe fn rounds_p (mut x : X8) -> X8 { const O1 : i64 = 0x0101_0101_0101_0101 ; let mut const_p = [_mm_cvtsi64_si128 (0) ; 14] ; for (i , p) in (0 ..) . zip (& mut const_p) { * p = _mm_set_epi64x ((i * O1) ^ 0xf0e0_d0c0_b0a0_9080u64 as i64 , (i * O1) ^ 0x7060_5040_3020_1000 ,) ; } let mask = X8 (_mm_set_epi64x (0x0306_090c_0f02_0508 , 0x0b0e_0104_070a_0d00) , _mm_set_epi64x (0x0407_0a0d_0003_0609 , 0x0c0f_0205_080b_0e01) , _mm_set_epi64x (0x0508_0b0e_0104_070a , 0x0d00_0306_090c_0f02) , _mm_set_epi64x (0x0609_0c0f_0205_080b , 0x0e01_0407_0a0d_0003) , _mm_set_epi64x (0x070a_0d00_0306_090c , 0x0f02_0508_0b0e_0104) , _mm_set_epi64x (0x080b_0e01_0407_0a0d , 0x0003_0609_0c0f_0205) , _mm_set_epi64x (0x090c_0f02_0508_0b0e , 0x0104_070a_0d00_0306) , _mm_set_epi64x (0x0e01_0407_0a0d_0003 , 0x0609_0c0f_0205_080b) ,) ; for p in const_p . chunks_exact (2) { x . 0 = _mm_xor_si128 (x . 0 , p [0]) ; x = (x , mask) . map (| x , m | _mm_shuffle_epi8 (x , m)) ; x = submix (x) ; x . 0 = _mm_xor_si128 (x . 0 , p [1]) ; x = (x , mask) . map (| x , m | _mm_shuffle_epi8 (x , m)) ; x = submix (x) ; } x }
};
}
