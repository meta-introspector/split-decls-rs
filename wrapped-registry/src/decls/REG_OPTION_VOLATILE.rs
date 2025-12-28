macro_rules! deps {
    () => {
        REG_OPEN_CREATE_OPTIONS!();
    };
}

macro_rules! REG_OPTION_VOLATILE {
    () => {
        deps!();
        pub const REG_OPTION_VOLATILE : REG_OPEN_CREATE_OPTIONS = 1u32 ;
    };
}

REG_OPTION_VOLATILE!()