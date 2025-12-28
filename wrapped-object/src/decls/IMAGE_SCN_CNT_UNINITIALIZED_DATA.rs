macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_CNT_UNINITIALIZED_DATA {
    () => {
        deps!();
        # [doc = " Section contains uninitialized data."] pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA : u32 = 0x0000_0080 ;
    };
}

IMAGE_SCN_CNT_UNINITIALIZED_DATA!();