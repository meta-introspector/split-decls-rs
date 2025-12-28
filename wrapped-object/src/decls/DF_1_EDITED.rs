macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_1_EDITED {
    () => {
        deps!();
        # [doc = " Object is modified after built."] pub const DF_1_EDITED : u32 = 0x0020_0000 ;
    };
}

DF_1_EDITED!()