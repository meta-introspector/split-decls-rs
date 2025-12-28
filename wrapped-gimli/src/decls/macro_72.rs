macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_endianity` attribute."] # [doc = ""] # [doc = " See Section 7.8, Table 7.13."] DwEnd (u8) { DW_END_default = 0x00 , DW_END_big = 0x01 , DW_END_little = 0x02 , DW_END_lo_user = 0x40 , DW_END_hi_user = 0xff , }) ;
    };
}

macro_72!();