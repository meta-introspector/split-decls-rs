macro_rules! SG_PROTECTED_VERSION_1 {
    () => {
        # [doc = " This segment is protected.  If the segment starts at file offset 0, the first page of the segment is not protected.  All other pages of the segment are protected."] pub const SG_PROTECTED_VERSION_1 : u32 = 0x8 ;
    };
}

SG_PROTECTED_VERSION_1!();