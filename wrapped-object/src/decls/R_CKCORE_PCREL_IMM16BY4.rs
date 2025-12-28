macro_rules! R_CKCORE_PCREL_IMM16BY4 {
    () => {
        # [doc = " disp ((S + A - P) >> 2) & 0xffff"] pub const R_CKCORE_PCREL_IMM16BY4 : u32 = 21 ;
    };
}

R_CKCORE_PCREL_IMM16BY4!();