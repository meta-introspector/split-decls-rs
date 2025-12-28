macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_WRITE {
    () => {
        deps!();
        # [doc = " Section is writeable."] pub const IMAGE_SCN_MEM_WRITE : u32 = 0x8000_0000 ;
    };
}

IMAGE_SCN_MEM_WRITE!()