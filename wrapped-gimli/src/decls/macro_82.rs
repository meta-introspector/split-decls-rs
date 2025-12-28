macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_ordering` attribute."] # [doc = ""] # [doc = " See Section 7.17, Table 7.17."] DwOrd (u8) { DW_ORD_row_major = 0x00 , DW_ORD_col_major = 0x01 , }) ;
    };
}

macro_82!();