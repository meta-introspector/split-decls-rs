macro_rules! ksx1001_other_decode {
    () => {
        # [inline (always)] pub fn ksx1001_other_decode (pointer : u16) -> u16 { map_with_ranges (& KSX1001_OTHER_POINTERS [.. KSX1001_OTHER_POINTERS . len () - 1] , & KSX1001_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
    };
}

ksx1001_other_decode!();