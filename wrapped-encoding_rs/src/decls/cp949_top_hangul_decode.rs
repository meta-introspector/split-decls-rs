macro_rules! cp949_top_hangul_decode {
    () => {
        # [inline (always)] pub fn cp949_top_hangul_decode (pointer : u16) -> u16 { map_with_ranges (& CP949_TOP_HANGUL_POINTERS [..] , & CP949_TOP_HANGUL_OFFSETS [..] , pointer ,) }
    };
}

cp949_top_hangul_decode!();