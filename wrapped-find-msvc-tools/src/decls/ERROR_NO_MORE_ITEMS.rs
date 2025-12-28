macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! ERROR_NO_MORE_ITEMS {
    () => {
        deps!();
        pub const ERROR_NO_MORE_ITEMS : WIN32_ERROR = 259u32 ;
    };
}

ERROR_NO_MORE_ITEMS!();