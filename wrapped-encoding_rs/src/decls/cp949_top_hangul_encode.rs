macro_rules! cp949_top_hangul_encode {
    () => {
        # [cfg (not (feature = "fast-hangul-encode"))] # [inline (always)] pub fn cp949_top_hangul_encode (bmp : u16) -> u16 { map_with_ranges (& CP949_TOP_HANGUL_OFFSETS [..] , & CP949_TOP_HANGUL_POINTERS [..] , bmp ,) }
    };
}

cp949_top_hangul_encode!()