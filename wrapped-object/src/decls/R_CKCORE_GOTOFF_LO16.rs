macro_rules! R_CKCORE_GOTOFF_LO16 {
    () => {
        # [doc = " (S + A - GOT) & 0xffff"] pub const R_CKCORE_GOTOFF_LO16 : u32 = 29 ;
    };
}

R_CKCORE_GOTOFF_LO16!();