macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_DWORD {
    () => {
        deps!();
        pub const REG_DWORD : REG_VALUE_TYPE = 4u32 ;
    };
}

REG_DWORD!()