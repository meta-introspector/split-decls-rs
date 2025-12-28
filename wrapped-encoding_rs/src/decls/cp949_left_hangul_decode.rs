macro_rules! cp949_left_hangul_decode {
    () => {
        # [inline (always)] pub fn cp949_left_hangul_decode (pointer : u16) -> u16 { map_with_ranges (& CP949_LEFT_HANGUL_POINTERS [..] , & CP949_LEFT_HANGUL_OFFSETS [..] , pointer ,) }
    };
}

cp949_left_hangul_decode!()