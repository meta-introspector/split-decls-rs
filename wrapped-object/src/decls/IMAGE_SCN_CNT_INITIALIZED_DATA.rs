macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_CNT_INITIALIZED_DATA {
    () => {
        deps!();
        # [doc = " Section contains initialized data."] pub const IMAGE_SCN_CNT_INITIALIZED_DATA : u32 = 0x0000_0040 ;
    };
}

IMAGE_SCN_CNT_INITIALIZED_DATA!()