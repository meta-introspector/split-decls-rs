macro_rules! gbk_other_decode {
    () => {
        # [inline (always)] pub fn gbk_other_decode (pointer : u16) -> u16 { map_with_ranges (& GBK_OTHER_POINTERS [.. GBK_OTHER_POINTERS . len () - 1] , & GBK_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
    };
}

gbk_other_decode!()