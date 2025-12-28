macro_rules! PPC_RELOC_LO14 {
    () => {
        # [doc = " Same as the LO16 except that the low 2 bits are not stored in the instruction and are"] # [doc = " always zero.  This is used in double word load/store instructions."] pub const PPC_RELOC_LO14 : u8 = 7 ;
    };
}

PPC_RELOC_LO14!();