macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_ARM64_SECTION {
    () => {
        deps!();
        # [doc = " Section table index"] pub const IMAGE_REL_ARM64_SECTION : u16 = 0x000D ;
    };
}

IMAGE_REL_ARM64_SECTION!();