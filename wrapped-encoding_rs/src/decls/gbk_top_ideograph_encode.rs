macro_rules! gbk_top_ideograph_encode {
    () => {
        # [cfg (not (feature = "fast-gb-hanzi-encode"))] # [inline (always)] pub fn gbk_top_ideograph_encode (bmp : u16) -> u16 { map_with_ranges (& GBK_TOP_IDEOGRAPH_OFFSETS [..] , & GBK_TOP_IDEOGRAPH_POINTERS [..] , bmp ,) }
    };
}

gbk_top_ideograph_encode!();