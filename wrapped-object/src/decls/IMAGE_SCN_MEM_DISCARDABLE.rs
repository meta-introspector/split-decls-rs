macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_MEM_DISCARDABLE {
    () => {
        deps!();
        # [doc = " Section can be discarded."] pub const IMAGE_SCN_MEM_DISCARDABLE : u32 = 0x0200_0000 ;
    };
}

IMAGE_SCN_MEM_DISCARDABLE!()