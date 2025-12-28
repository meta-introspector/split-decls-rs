macro_rules! IMAGE_DLLCHARACTERISTICS_NO_SEH {
    () => {
        # [doc = " Image does not use SEH.  No SE handler may reside in this image"] pub const IMAGE_DLLCHARACTERISTICS_NO_SEH : u16 = 0x0400 ;
    };
}

IMAGE_DLLCHARACTERISTICS_NO_SEH!();