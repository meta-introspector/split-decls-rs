macro_rules! shift_jis_to_iso_2022_jp {
    () => {
        # [cfg (any (feature = "less-slow-kanji-encode" , feature = "fast-kanji-encode"))] # [inline (always)] fn shift_jis_to_iso_2022_jp (tuple : (u8 , u8)) -> (u8 , u8) { let (shift_jis_lead , shift_jis_trail) = tuple ; let mut lead = shift_jis_lead as usize ; if shift_jis_lead >= 0xA0 { lead -= 0xC1 - 0x81 ; } lead <<= 1 ; lead -= 0xE1 ; let trail = if shift_jis_trail >= 0x9F { lead += 1 ; shift_jis_trail - (0x9F - 0x21) } else if shift_jis_trail < 0x7F { shift_jis_trail - (0x40 - 0x21) } else { shift_jis_trail - (0x41 - 0x21) } ; (lead as u8 , trail) }
    };
}

shift_jis_to_iso_2022_jp!()