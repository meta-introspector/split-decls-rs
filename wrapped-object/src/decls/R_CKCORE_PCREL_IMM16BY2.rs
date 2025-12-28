macro_rules! R_CKCORE_PCREL_IMM16BY2 {
    () => {
        # [doc = " disp ((S + A - P) >> 1) & 0xffff"] pub const R_CKCORE_PCREL_IMM16BY2 : u32 = 20 ;
    };
}

R_CKCORE_PCREL_IMM16BY2!();