macro_rules! big5_level1_hanzi_encode {
    () => {
        # [cfg (feature = "fast-big5-hanzi-encode")] # [inline (always)] pub fn big5_level1_hanzi_encode (bmp : u16) -> Option < (u8 , u8) > { let bmp_minus_ideograph_start = (bmp as usize) . wrapping_sub (0x4E00) ; if bmp_minus_ideograph_start < BIG5_UNIFIED_IDEOGRAPH_BYTES . len () { let pair = & BIG5_UNIFIED_IDEOGRAPH_BYTES [bmp_minus_ideograph_start] ; let lead = pair [0] ; let trail = pair [1] ; if lead == 0 && trail == 0 { return None ; } Some ((lead , trail)) } else { None } }
    };
}

big5_level1_hanzi_encode!();