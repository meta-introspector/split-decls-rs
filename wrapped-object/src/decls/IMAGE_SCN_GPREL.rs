macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_GPREL {
    () => {
        deps!();
        # [doc = " Section content can be accessed relative to GP"] pub const IMAGE_SCN_GPREL : u32 = 0x0000_8000 ;
    };
}

IMAGE_SCN_GPREL!();