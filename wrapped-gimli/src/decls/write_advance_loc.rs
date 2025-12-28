macro_rules! deps {
    () => {
        Result!();
        Writer!();
    };
}

macro_rules! write_advance_loc {
    () => {
        deps!();
        fn write_advance_loc < W : Writer > (w : & mut W , code_alignment_factor : u8 , prev_offset : u32 , offset : u32 ,) -> Result < () > { if offset == prev_offset { return Ok (()) ; } let delta = factored_code_delta (prev_offset , offset , code_alignment_factor) ? ; if delta < 0x40 { w . write_u8 (constants :: DW_CFA_advance_loc . 0 | delta as u8) ? ; } else if delta < 0x100 { w . write_u8 (constants :: DW_CFA_advance_loc1 . 0) ? ; w . write_u8 (delta as u8) ? ; } else if delta < 0x10000 { w . write_u8 (constants :: DW_CFA_advance_loc2 . 0) ? ; w . write_u16 (delta as u16) ? ; } else { w . write_u8 (constants :: DW_CFA_advance_loc4 . 0) ? ; w . write_u32 (delta) ? ; } Ok (()) }
    };
}

write_advance_loc!()