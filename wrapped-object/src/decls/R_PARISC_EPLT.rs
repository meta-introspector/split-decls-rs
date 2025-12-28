macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! R_PARISC_EPLT {
    () => {
        deps!();
        # [doc = " Dynamic reloc, exported PLT"] pub const R_PARISC_EPLT : u32 = 130 ;
    };
}

R_PARISC_EPLT!();