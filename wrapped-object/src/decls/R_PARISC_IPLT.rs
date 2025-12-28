macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! R_PARISC_IPLT {
    () => {
        deps!();
        # [doc = " Dynamic reloc, imported PLT"] pub const R_PARISC_IPLT : u32 = 129 ;
    };
}

R_PARISC_IPLT!()