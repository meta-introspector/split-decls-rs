macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_NOT_CACHED {
    () => {
        deps!();
        # [doc = " Section is not cacheable."] pub const IMAGE_SCN_MEM_NOT_CACHED : u32 = 0x0400_0000 ;
    };
}

IMAGE_SCN_MEM_NOT_CACHED!();