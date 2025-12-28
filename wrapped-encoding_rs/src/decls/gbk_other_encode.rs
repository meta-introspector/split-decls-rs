macro_rules! gbk_other_encode {
    () => {
        # [inline (always)] pub fn gbk_other_encode (bmp : u16) -> Option < u16 > { map_with_unsorted_ranges (& GBK_OTHER_UNSORTED_OFFSETS [..] , & GBK_OTHER_POINTERS [..] , bmp ,) }
    };
}

gbk_other_encode!();