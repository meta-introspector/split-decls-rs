macro_rules! shift_jis_to_euc_jp {
    () => {
        # [cfg (any (feature = "less-slow-kanji-encode" , feature = "fast-kanji-encode"))] # [inline (always)] fn shift_jis_to_euc_jp (tuple : (u8 , u8)) -> (u8 , u8) { let (shift_jis_lead , shift_jis_trail) = tuple ; let mut lead = shift_jis_lead as usize ; if shift_jis_lead >= 0xA0 { lead -= 0xC1 - 0x81 ; } lead <<= 1 ; lead -= 0x61 ; let trail = if shift_jis_trail >= 0x9F { lead += 1 ; shift_jis_trail + (0xA1 - 0x9F) } else if shift_jis_trail < 0x7F { shift_jis_trail + (0xA1 - 0x40) } else { shift_jis_trail + (0xA1 - 0x41) } ; (lead as u8 , trail) }
    };
}

shift_jis_to_euc_jp!()