macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_SHARED {
    () => {
        deps!();
        # [doc = " Section is shareable."] pub const IMAGE_SCN_MEM_SHARED : u32 = 0x1000_0000 ;
    };
}

IMAGE_SCN_MEM_SHARED!()