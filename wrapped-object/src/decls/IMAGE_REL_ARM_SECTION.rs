macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_ARM_SECTION {
    () => {
        deps!();
        # [doc = " Section table index"] pub const IMAGE_REL_ARM_SECTION : u16 = 0x000E ;
    };
}

IMAGE_REL_ARM_SECTION!()