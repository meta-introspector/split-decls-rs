macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        dw ! (# [doc = " The section type field in a `.dwp` unit index."] # [doc = ""] # [doc = " This is used for version 5 and later."] # [doc = ""] # [doc = " See Section 7.3.5."] DwSect (u32) { DW_SECT_INFO = 1 , DW_SECT_ABBREV = 3 , DW_SECT_LINE = 4 , DW_SECT_LOCLISTS = 5 , DW_SECT_STR_OFFSETS = 6 , DW_SECT_MACRO = 7 , DW_SECT_RNGLISTS = 8 , }) ;
    };
}

macro_61!()