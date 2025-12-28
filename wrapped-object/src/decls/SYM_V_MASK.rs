macro_rules! SYM_V_MASK {
    () => {
        # [doc = " Values for visibility as they would appear when encoded in the high 4 bits"] # [doc = " of the 16-bit unsigned n_type field of symbol table entries. Valid for"] # [doc = " 32-bit XCOFF only when the o_vstamp in the auxiliary header is greater than 1."] pub const SYM_V_MASK : u16 = 0xF000 ;
    };
}

SYM_V_MASK!();