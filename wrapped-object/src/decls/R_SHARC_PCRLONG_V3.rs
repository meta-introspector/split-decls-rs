macro_rules! R_SHARC_PCRLONG_V3 {
    () => {
        # [doc = " 24-bit PC-relative address in bits 23:0 of a 48-bit instr"] # [doc = ""] # [doc = " Targets:"] # [doc = ""] # [doc = " * Type 8a"] # [doc = " * Type 12a (truncated to 23 bits after relocation)"] # [doc = " * Type 13a (truncated to 23 bits after relocation)"] # [doc = " * Type 25a (PC Relative)"] pub const R_SHARC_PCRLONG_V3 : u32 = 0x0f ;
    };
}

R_SHARC_PCRLONG_V3!();