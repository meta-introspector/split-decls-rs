macro_rules! encode_kanji {
    () => {
        # [cfg (not (feature = "fast-kanji-encode"))] # [inline (always)] fn encode_kanji (bmp : u16) -> Option < (u8 , u8) > { if let Some ((lead , trail)) = jis0208_level1_kanji_shift_jis_encode (bmp) { return Some ((lead , trail)) ; } let pointer = if 0x4EDD == bmp { 23 } else if let Some (pos) = jis0208_level2_and_additional_kanji_encode (bmp) { 4418 + pos } else if let Some (pos) = position (& IBM_KANJI [..] , bmp) { 10744 + pos } else { return None ; } ; let lead = pointer / 188 ; let lead_offset = if lead < 0x1F { 0x81usize } else { 0xC1usize } ; let trail = pointer % 188 ; let trail_offset = if trail < 0x3F { 0x40usize } else { 0x41usize } ; Some (((lead + lead_offset) as u8 , (trail + trail_offset) as u8)) }
    };
}

encode_kanji!()