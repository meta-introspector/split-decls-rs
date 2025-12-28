macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_79 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_identifier_case` attribute."] # [doc = ""] # [doc = " See Section 7.14, Table 7.18."] DwId (u8) { DW_ID_case_sensitive = 0x00 , DW_ID_up_case = 0x01 , DW_ID_down_case = 0x02 , DW_ID_case_insensitive = 0x03 , }) ;
    };
}

macro_79!()