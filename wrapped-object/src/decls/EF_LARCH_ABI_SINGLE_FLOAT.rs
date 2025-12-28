macro_rules! EF_LARCH_ABI_SINGLE_FLOAT {
    () => {
        # [doc = " Uses GPRs, 32-bit FPRs and the stack for parameter passing"] pub const EF_LARCH_ABI_SINGLE_FLOAT : u32 = 0x2 ;
    };
}

EF_LARCH_ABI_SINGLE_FLOAT!();