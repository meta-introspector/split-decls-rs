macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_MULTI_SZ {
    () => {
        deps!();
        pub const REG_MULTI_SZ : REG_VALUE_TYPE = 7u32 ;
    };
}

REG_MULTI_SZ!();