macro_rules! gb18030_range_decode {
    () => {
        # [inline (always)] pub fn gb18030_range_decode (pointer : u16) -> u16 { map_with_ranges (& GB18030_RANGE_POINTERS [..] , & GB18030_RANGE_OFFSETS [..] , pointer ,) }
    };
}

gb18030_range_decode!();