macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_83 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_discr_list` attribute."] # [doc = ""] # [doc = " See Section 7.18, Table 7.22."] DwDsc (u8) { DW_DSC_label = 0x00 , DW_DSC_range = 0x01 , }) ;
    };
}

macro_83!()