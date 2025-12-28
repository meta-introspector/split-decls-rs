macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_READ {
    () => {
        deps!();
        # [doc = " Section is readable."] pub const IMAGE_SCN_MEM_READ : u32 = 0x4000_0000 ;
    };
}

IMAGE_SCN_MEM_READ!();