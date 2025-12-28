macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_BINARY {
    () => {
        deps!();
        pub const REG_BINARY : REG_VALUE_TYPE = 3u32 ;
    };
}

REG_BINARY!();