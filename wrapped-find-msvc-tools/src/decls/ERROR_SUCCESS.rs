macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! ERROR_SUCCESS {
    () => {
        deps!();
        pub const ERROR_SUCCESS : WIN32_ERROR = 0u32 ;
    };
}

ERROR_SUCCESS!();