macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHT_NULL {
    () => {
        deps!();
        # [doc = " Section header table entry is unused."] pub const SHT_NULL : u32 = 0 ;
    };
}

SHT_NULL!();