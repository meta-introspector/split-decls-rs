macro_rules! R_CKCORE_ADDRPLT_HI16 {
    () => {
        # [doc = " high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF"] pub const R_CKCORE_ADDRPLT_HI16 : u32 = 38 ;
    };
}

R_CKCORE_ADDRPLT_HI16!();