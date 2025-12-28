macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! SHT_DYNAMIC {
    () => {
        deps!();
        # [doc = " Dynamic linking information."] pub const SHT_DYNAMIC : u32 = 6 ;
    };
}

SHT_DYNAMIC!()