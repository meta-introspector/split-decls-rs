macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_M32R_SECTION {
    () => {
        deps!();
        # [doc = " Section table index"] pub const IMAGE_REL_M32R_SECTION : u16 = 0x000C ;
    };
}

IMAGE_REL_M32R_SECTION!();