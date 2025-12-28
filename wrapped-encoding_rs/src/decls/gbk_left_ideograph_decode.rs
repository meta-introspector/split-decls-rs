macro_rules! gbk_left_ideograph_decode {
    () => {
        # [inline (always)] pub fn gbk_left_ideograph_decode (pointer : u16) -> u16 { map_with_ranges (& GBK_LEFT_IDEOGRAPH_POINTERS [..] , & GBK_LEFT_IDEOGRAPH_OFFSETS [..] , pointer ,) }
    };
}

gbk_left_ideograph_decode!();