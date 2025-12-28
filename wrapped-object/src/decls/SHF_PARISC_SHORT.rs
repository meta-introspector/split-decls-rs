macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_PARISC_SHORT {
    () => {
        deps!();
        # [doc = " Section with short addressing."] pub const SHF_PARISC_SHORT : u32 = 0x2000_0000 ;
    };
}

SHF_PARISC_SHORT!();