macro_rules! deps {
    () => {
        Architecture!();
    };
}

macro_rules! EF_PARISC_ARCH {
    () => {
        deps!();
        # [doc = " Architecture version."] pub const EF_PARISC_ARCH : u32 = 0x0000_ffff ;
    };
}

EF_PARISC_ARCH!();