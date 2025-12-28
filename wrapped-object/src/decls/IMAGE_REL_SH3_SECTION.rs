macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_SH3_SECTION {
    () => {
        deps!();
        # [doc = " Section table index"] pub const IMAGE_REL_SH3_SECTION : u16 = 0x000E ;
    };
}

IMAGE_REL_SH3_SECTION!();