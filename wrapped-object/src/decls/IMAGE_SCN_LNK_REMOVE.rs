macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_LNK_REMOVE {
    () => {
        deps!();
        # [doc = " Section contents will not become part of image."] pub const IMAGE_SCN_LNK_REMOVE : u32 = 0x0000_0800 ;
    };
}

IMAGE_SCN_LNK_REMOVE!();