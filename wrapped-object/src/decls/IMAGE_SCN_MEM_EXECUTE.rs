macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_EXECUTE {
    () => {
        deps!();
        # [doc = " Section is executable."] pub const IMAGE_SCN_MEM_EXECUTE : u32 = 0x2000_0000 ;
    };
}

IMAGE_SCN_MEM_EXECUTE!()