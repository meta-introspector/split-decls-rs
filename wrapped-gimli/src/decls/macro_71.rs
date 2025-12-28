macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_decimal_sign` attribute."] # [doc = ""] # [doc = " See Section 7.8, Table 7.12."] DwDs (u8) { DW_DS_unsigned = 0x01 , DW_DS_leading_overpunch = 0x02 , DW_DS_trailing_overpunch = 0x03 , DW_DS_leading_separate = 0x04 , DW_DS_trailing_separate = 0x05 , }) ;
    };
}

macro_71!()