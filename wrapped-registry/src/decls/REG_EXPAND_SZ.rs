macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_EXPAND_SZ {
    () => {
        deps!();
        pub const REG_EXPAND_SZ : REG_VALUE_TYPE = 2u32 ;
    };
}

REG_EXPAND_SZ!();