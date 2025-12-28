macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_SZ {
    () => {
        deps!();
        pub const REG_SZ : REG_VALUE_TYPE = 1u32 ;
    };
}

REG_SZ!()