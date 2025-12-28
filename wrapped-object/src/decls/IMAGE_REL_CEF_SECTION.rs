macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_CEF_SECTION {
    () => {
        deps!();
        # [doc = " Section index"] pub const IMAGE_REL_CEF_SECTION : u16 = 0x0004 ;
    };
}

IMAGE_REL_CEF_SECTION!();