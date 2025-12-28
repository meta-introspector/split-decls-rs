macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_85 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_defaulted` attribute."] # [doc = ""] # [doc = " See Section 7.20, Table 7.24."] DwDefaulted (u8) { DW_DEFAULTED_no = 0x00 , DW_DEFAULTED_in_class = 0x01 , DW_DEFAULTED_out_of_class = 0x02 , }) ;
    };
}

macro_85!()