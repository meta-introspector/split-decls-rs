macro_rules! gb2312_other_decode {
    () => {
        # [inline (always)] pub fn gb2312_other_decode (pointer : u16) -> u16 { map_with_ranges (& GB2312_OTHER_POINTERS [.. GB2312_OTHER_POINTERS . len () - 1] , & GB2312_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
    };
}

gb2312_other_decode!();