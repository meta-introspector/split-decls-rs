macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_GNU_RETAIN {
    () => {
        deps!();
        # [doc = " Section should not be garbage collected by the linker."] pub const SHF_GNU_RETAIN : u32 = 1 << 21 ;
    };
}

SHF_GNU_RETAIN!();