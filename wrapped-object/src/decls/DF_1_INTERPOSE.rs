macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_1_INTERPOSE {
    () => {
        deps!();
        # [doc = " Object is used to interpose."] pub const DF_1_INTERPOSE : u32 = 0x0000_0400 ;
    };
}

DF_1_INTERPOSE!();