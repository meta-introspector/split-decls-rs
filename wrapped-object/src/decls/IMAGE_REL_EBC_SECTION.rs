macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_REL_EBC_SECTION {
    () => {
        deps!();
        # [doc = " Section table index"] pub const IMAGE_REL_EBC_SECTION : u16 = 0x0003 ;
    };
}

IMAGE_REL_EBC_SECTION!()