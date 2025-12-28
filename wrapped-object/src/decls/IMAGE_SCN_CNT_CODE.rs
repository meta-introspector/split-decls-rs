macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_CNT_CODE {
    () => {
        deps!();
        # [doc = " Section contains code."] pub const IMAGE_SCN_CNT_CODE : u32 = 0x0000_0020 ;
    };
}

IMAGE_SCN_CNT_CODE!()