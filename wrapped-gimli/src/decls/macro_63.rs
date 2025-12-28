macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        dw ! (# [doc = " The unit type field in a unit header."] # [doc = ""] # [doc = " See Section 7.5.1, Table 7.2."] DwUt (u8) { DW_UT_compile = 0x01 , DW_UT_type = 0x02 , DW_UT_partial = 0x03 , DW_UT_skeleton = 0x04 , DW_UT_split_compile = 0x05 , DW_UT_split_type = 0x06 , DW_UT_lo_user = 0x80 , DW_UT_hi_user = 0xff , }) ;
    };
}

macro_63!()