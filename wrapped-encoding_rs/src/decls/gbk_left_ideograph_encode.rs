macro_rules! gbk_left_ideograph_encode {
    () => {
        # [cfg (not (feature = "fast-gb-hanzi-encode"))] # [inline (always)] pub fn gbk_left_ideograph_encode (bmp : u16) -> u16 { map_with_ranges (& GBK_LEFT_IDEOGRAPH_OFFSETS [..] , & GBK_LEFT_IDEOGRAPH_POINTERS [..] , bmp ,) }
    };
}

gbk_left_ideograph_encode!();