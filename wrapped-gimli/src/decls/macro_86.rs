macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_86 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings for the standard opcodes for line number information."] # [doc = ""] # [doc = " See Section 7.22, Table 7.25."] DwLns (u8) { DW_LNS_copy = 0x01 , DW_LNS_advance_pc = 0x02 , DW_LNS_advance_line = 0x03 , DW_LNS_set_file = 0x04 , DW_LNS_set_column = 0x05 , DW_LNS_negate_stmt = 0x06 , DW_LNS_set_basic_block = 0x07 , DW_LNS_const_add_pc = 0x08 , DW_LNS_fixed_advance_pc = 0x09 , DW_LNS_set_prologue_end = 0x0a , DW_LNS_set_epilogue_begin = 0x0b , DW_LNS_set_isa = 0x0c , }) ;
    };
}

macro_86!();