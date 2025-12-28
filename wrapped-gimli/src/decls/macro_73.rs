macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_accessibility` attribute."] # [doc = ""] # [doc = " See Section 7.9, Table 7.14."] DwAccess (u8) { DW_ACCESS_public = 0x01 , DW_ACCESS_protected = 0x02 , DW_ACCESS_private = 0x03 , }) ;
    };
}

macro_73!();