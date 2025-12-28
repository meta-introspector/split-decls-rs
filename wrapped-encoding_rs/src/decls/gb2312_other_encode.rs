macro_rules! gb2312_other_encode {
    () => {
        # [inline (always)] pub fn gb2312_other_encode (bmp : u16) -> Option < u16 > { map_with_unsorted_ranges (& GB2312_OTHER_UNSORTED_OFFSETS [..] , & GB2312_OTHER_POINTERS [..] , bmp ,) }
    };
}

gb2312_other_encode!();