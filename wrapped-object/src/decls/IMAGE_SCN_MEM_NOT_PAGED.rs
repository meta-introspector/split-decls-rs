macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_NOT_PAGED {
    () => {
        deps!();
        # [doc = " Section is not pageable."] pub const IMAGE_SCN_MEM_NOT_PAGED : u32 = 0x0800_0000 ;
    };
}

IMAGE_SCN_MEM_NOT_PAGED!()