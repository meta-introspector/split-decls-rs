macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! PT_DYNAMIC {
    () => {
        deps!();
        # [doc = " Dynamic linking information."] pub const PT_DYNAMIC : u32 = 2 ;
    };
}

PT_DYNAMIC!();