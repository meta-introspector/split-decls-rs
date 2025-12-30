// Generated macro for round (function)
macro_rules! Depcrate_compressorround {
() => {
// Module: crate::compressor
// Provides: {"round"}
// Dependencies: {}
# [inline (always)] unsafe fn round (i : i64 , a : X8) -> X8 { let ff = 0xffff_ffff_ffff_ffffu64 as i64 ; let l0 = _mm_set_epi64x (ff , (i * 0x0101_0101_0101_0101) ^ 0x7060_5040_3020_1000) ; let lx = _mm_set_epi64x (ff , 0) ; let l7 = _mm_set_epi64x ((i * 0x0101_0101_0101_0101) ^ 0x8f9f_afbf_cfdf_efffu64 as i64 , 0 ,) ; let a = a ^ X8 (l0 , lx , lx , lx , lx , lx , lx , l7) ; let mask = X8 (_mm_set_epi64x (0x0306_0a0d_0802_0509 , 0x0c0f_0104_070b_0e00) , _mm_set_epi64x (0x0407_0c0f_0a03_060b , 0x0e09_0205_000d_0801) , _mm_set_epi64x (0x0500_0e09_0c04_070d , 0x080b_0306_010f_0a02) , _mm_set_epi64x (0x0601_080b_0e05_000f , 0x0a0d_0407_0209_0c03) , _mm_set_epi64x (0x0702_090c_0f06_0108 , 0x0b0e_0500_030a_0d04) , _mm_set_epi64x (0x0003_0b0e_0907_020a , 0x0d08_0601_040c_0f05) , _mm_set_epi64x (0x0104_0d08_0b00_030c , 0x0f0a_0702_050e_0906) , _mm_set_epi64x (0x0205_0f0a_0d01_040e , 0x090c_0003_0608_0b07) ,) ; let a = (a , mask) . map (| x , y | _mm_shuffle_epi8 (x , y)) ; submix (a) }
};
}
