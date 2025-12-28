macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHN_PARISC_ANSI_COMMON {
    () => {
        deps!();
        # [doc = " Section for tentatively declared symbols in ANSI C."] pub const SHN_PARISC_ANSI_COMMON : u16 = 0xff00 ;
    };
}

SHN_PARISC_ANSI_COMMON!();