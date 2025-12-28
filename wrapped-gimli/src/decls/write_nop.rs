macro_rules! deps {
    () => {
        Result!();
        Writer!();
    };
}

macro_rules! write_nop {
    () => {
        deps!();
        fn write_nop < W : Writer > (w : & mut W , len : usize , align : u8) -> Result < () > { debug_assert_eq ! (align & (align - 1) , 0) ; let tail_len = (! len + 1) & (align as usize - 1) ; for _ in 0 .. tail_len { w . write_u8 (constants :: DW_CFA_nop . 0) ? ; } Ok (()) }
    };
}

write_nop!()