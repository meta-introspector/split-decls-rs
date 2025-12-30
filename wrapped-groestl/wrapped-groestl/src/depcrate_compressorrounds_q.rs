// Generated macro for rounds_q (function)
macro_rules! Depcrate_compressorrounds_q {
() => {
// Module: crate::compressor
// Provides: {"rounds_q"}
// Dependencies: {}
# [inline (always)] unsafe fn rounds_q (mut x : X8) -> X8 { const O1 : i64 = 0x0101_0101_0101_0101 ; let mut const_q = [_mm_cvtsi64_si128 (0) ; 14] ; for (i , q) in (0 ..) . zip (& mut const_q) { * q = _mm_set_epi64x ((i * O1) ^ 0x0f1f_2f3f_4f5f_6f7f , (i * O1) ^ 0x8f9f_afbf_cfdf_efffu64 as i64 ,) ; } let mask = X8 (_mm_set_epi64x (0x0306_090c_0f02_0508 , 0x0b0e_0104_070a_0d00) , _mm_set_epi64x (0x0407_0a0d_0003_0609 , 0x0c0f_0205_080b_0e01) , _mm_set_epi64x (0x0508_0b0e_0104_070a , 0x0d00_0306_090c_0f02) , _mm_set_epi64x (0x0609_0c0f_0205_080b , 0x0e01_0407_0a0d_0003) , _mm_set_epi64x (0x070a_0d00_0306_090c , 0x0f02_0508_0b0e_0104) , _mm_set_epi64x (0x080b_0e01_0407_0a0d , 0x0003_0609_0c0f_0205) , _mm_set_epi64x (0x090c_0f02_0508_0b0e , 0x0104_070a_0d00_0306) , _mm_set_epi64x (0x0e01_0407_0a0d_0003 , 0x0609_0c0f_0205_080b) ,) . shuffle ((1 , 3 , 5 , 7 , 0 , 2 , 4 , 6)) ; let f = _mm_set1_epi64x (0xffff_ffff_ffff_ffffu64 as i64) ; for q in const_q . chunks_exact (2) { x = (x ^ X8 (f , f , f , f , f , f , f , q [0]) , mask) . map (| x , m | _mm_shuffle_epi8 (x , m)) ; x = submix (x) ; x = (x ^ X8 (f , f , f , f , f , f , f , q [1]) , mask) . map (| x , m | _mm_shuffle_epi8 (x , m)) ; x = submix (x) ; } x }
};
}
