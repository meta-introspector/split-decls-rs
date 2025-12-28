macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! R_MICROBLAZE_32_SYM_OP_SYM {
    () => {
        deps!();
        # [doc = " Symbol Op Symbol relocation."] pub const R_MICROBLAZE_32_SYM_OP_SYM : u32 = 10 ;
    };
}

R_MICROBLAZE_32_SYM_OP_SYM!();