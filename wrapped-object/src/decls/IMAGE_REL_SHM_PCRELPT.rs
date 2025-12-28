macro_rules! IMAGE_REL_SHM_PCRELPT {
    () => {
        # [doc = " Offset from current instruction in longwords"] # [doc = " if not NOMODE, insert the inverse of the low bit at bit 32 to select PTA/PTB"] pub const IMAGE_REL_SHM_PCRELPT : u16 = 0x0013 ;
    };
}

IMAGE_REL_SHM_PCRELPT!();