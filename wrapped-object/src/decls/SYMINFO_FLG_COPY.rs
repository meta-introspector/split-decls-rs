macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SYMINFO_FLG_COPY {
    () => {
        deps!();
        # [doc = " Symbol is a copy-reloc"] pub const SYMINFO_FLG_COPY : u16 = 0x0004 ;
    };
}

SYMINFO_FLG_COPY!()