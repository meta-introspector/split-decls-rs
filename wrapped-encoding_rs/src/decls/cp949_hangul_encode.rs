macro_rules! cp949_hangul_encode {
    () => {
        # [cfg (feature = "fast-hangul-encode")] # [inline (always)] pub fn cp949_hangul_encode (bmp_minus_start : u16) -> (u8 , u8) { let pair = & CP949_HANGUL_BYTES [bmp_minus_start as usize] ; (pair [0] , pair [1]) }
    };
}

cp949_hangul_encode!()