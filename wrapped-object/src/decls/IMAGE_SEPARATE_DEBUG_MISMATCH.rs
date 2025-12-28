macro_rules! IMAGE_SEPARATE_DEBUG_MISMATCH {
    () => {
        # [doc = " when DBG was updated, the old checksum didn't match."] pub const IMAGE_SEPARATE_DEBUG_MISMATCH : u16 = 0x8000 ;
    };
}

IMAGE_SEPARATE_DEBUG_MISMATCH!();