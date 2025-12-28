macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! ERROR_INVALID_DATA {
    () => {
        deps!();
        pub const ERROR_INVALID_DATA : WIN32_ERROR = 13u32 ;
    };
}

ERROR_INVALID_DATA!()