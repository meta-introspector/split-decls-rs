macro_rules! deps {
    () => {
        REG_VALUE_TYPE!();
    };
}

macro_rules! REG_QWORD {
    () => {
        deps!();
        pub const REG_QWORD : REG_VALUE_TYPE = 11u32 ;
    };
}

REG_QWORD!();