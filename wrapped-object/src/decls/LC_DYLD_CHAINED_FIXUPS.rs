macro_rules! deps {
    () => {
        LinkeditDataCommand!();
    };
}

macro_rules! LC_DYLD_CHAINED_FIXUPS {
    () => {
        deps!();
        # [doc = " used with `LinkeditDataCommand`"] pub const LC_DYLD_CHAINED_FIXUPS : u32 = 0x34 | LC_REQ_DYLD ;
    };
}

LC_DYLD_CHAINED_FIXUPS!();