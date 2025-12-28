macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_AMD64_SECTION {
    () => {
        deps!();
        # [doc = " Section index"] pub const IMAGE_REL_AMD64_SECTION : u16 = 0x000A ;
    };
}

IMAGE_REL_AMD64_SECTION!();