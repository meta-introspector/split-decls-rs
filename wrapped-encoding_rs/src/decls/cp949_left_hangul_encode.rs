macro_rules! cp949_left_hangul_encode {
    () => {
        # [cfg (not (feature = "fast-hangul-encode"))] # [inline (always)] pub fn cp949_left_hangul_encode (bmp : u16) -> u16 { map_with_ranges (& CP949_LEFT_HANGUL_OFFSETS [..] , & CP949_LEFT_HANGUL_POINTERS [..] , bmp ,) }
    };
}

cp949_left_hangul_encode!();