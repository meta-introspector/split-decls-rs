macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_visibility` attribute."] # [doc = ""] # [doc = " See Section 7.10, Table 7.15."] DwVis (u8) { DW_VIS_local = 0x01 , DW_VIS_exported = 0x02 , DW_VIS_qualified = 0x03 , }) ;
    };
}

macro_74!();