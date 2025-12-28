macro_rules! R_CKCORE_PCREL_IMM10BY4 {
    () => {
        # [doc = " disp ((S + A - P) >> 2) & 0x3ff"] pub const R_CKCORE_PCREL_IMM10BY4 : u32 = 23 ;
    };
}

R_CKCORE_PCREL_IMM10BY4!();