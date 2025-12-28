macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        dw ! (# [doc = " Name index attribute encodings."] # [doc = ""] # [doc = " See Section 7.19, Table 7.23."] DwIdx (u16) { DW_IDX_compile_unit = 1 , DW_IDX_type_unit = 2 , DW_IDX_die_offset = 3 , DW_IDX_parent = 4 , DW_IDX_type_hash = 5 , DW_IDX_lo_user = 0x2000 , DW_IDX_hi_user = 0x3fff , }) ;
    };
}

macro_84!()