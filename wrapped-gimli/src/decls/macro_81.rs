macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_81 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_inline` attribute."] # [doc = ""] # [doc = " See Section 7.16, Table 7.20."] DwInl (u8) { DW_INL_not_inlined = 0x00 , DW_INL_inlined = 0x01 , DW_INL_declared_not_inlined = 0x02 , DW_INL_declared_inlined = 0x03 , }) ;
    };
}

macro_81!()