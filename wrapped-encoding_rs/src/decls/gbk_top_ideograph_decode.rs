macro_rules! gbk_top_ideograph_decode {
    () => {
        # [inline (always)] pub fn gbk_top_ideograph_decode (pointer : u16) -> u16 { map_with_ranges (& GBK_TOP_IDEOGRAPH_POINTERS [..] , & GBK_TOP_IDEOGRAPH_OFFSETS [..] , pointer ,) }
    };
}

gbk_top_ideograph_decode!()