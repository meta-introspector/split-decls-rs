macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_ORIGIN {
    () => {
        deps!();
        # [doc = " Object may use DF_ORIGIN"] pub const DF_ORIGIN : u32 = 0x0000_0001 ;
    };
}

DF_ORIGIN!();