macro_rules! ksx1001_other_encode {
    () => {
        # [inline (always)] pub fn ksx1001_other_encode (bmp : u16) -> Option < u16 > { map_with_unsorted_ranges (& KSX1001_OTHER_UNSORTED_OFFSETS [..] , & KSX1001_OTHER_POINTERS [..] , bmp ,) }
    };
}

ksx1001_other_encode!();