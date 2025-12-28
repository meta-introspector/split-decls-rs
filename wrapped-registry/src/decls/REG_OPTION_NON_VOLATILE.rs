macro_rules! deps {
    () => {
        REG_OPEN_CREATE_OPTIONS!();
    };
}

macro_rules! REG_OPTION_NON_VOLATILE {
    () => {
        deps!();
        pub const REG_OPTION_NON_VOLATILE : REG_OPEN_CREATE_OPTIONS = 0u32 ;
    };
}

REG_OPTION_NON_VOLATILE!()