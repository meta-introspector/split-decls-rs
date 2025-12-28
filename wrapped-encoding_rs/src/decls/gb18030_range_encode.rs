macro_rules! gb18030_range_encode {
    () => {
        # [inline (always)] pub fn gb18030_range_encode (bmp : u16) -> usize { if bmp == 0xE7C7 { return 7457 ; } map_with_ranges (& GB18030_RANGE_OFFSETS [..] , & GB18030_RANGE_POINTERS [..] , bmp) as usize }
    };
}

gb18030_range_encode!()