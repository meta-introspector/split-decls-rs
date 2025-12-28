macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_1_NODIRECT {
    () => {
        deps!();
        # [doc = " Object has no-direct binding."] pub const DF_1_NODIRECT : u32 = 0x0002_0000 ;
    };
}

DF_1_NODIRECT!();