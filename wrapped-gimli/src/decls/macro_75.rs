macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_virtuality` attribute."] # [doc = ""] # [doc = " See Section 7.11, Table 7.16."] DwVirtuality (u8) { DW_VIRTUALITY_none = 0x00 , DW_VIRTUALITY_virtual = 0x01 , DW_VIRTUALITY_pure_virtual = 0x02 , }) ;
    };
}

macro_75!()