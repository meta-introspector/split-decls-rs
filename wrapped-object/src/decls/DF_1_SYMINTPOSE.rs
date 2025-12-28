macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_1_SYMINTPOSE {
    () => {
        deps!();
        # [doc = " Object has individual interposers."] pub const DF_1_SYMINTPOSE : u32 = 0x0080_0000 ;
    };
}

DF_1_SYMINTPOSE!()