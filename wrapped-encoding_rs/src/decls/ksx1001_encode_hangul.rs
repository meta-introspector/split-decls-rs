macro_rules! ksx1001_encode_hangul {
    () => {
        # [cfg (feature = "fast-hangul-encode")] # [inline (always)] fn ksx1001_encode_hangul (_ : u16 , bmp_minus_hangul_start : u16) -> (u8 , u8) { cp949_hangul_encode (bmp_minus_hangul_start) }
    };
}

ksx1001_encode_hangul!()