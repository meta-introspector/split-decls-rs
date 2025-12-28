macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! ERROR_NO_UNICODE_TRANSLATION {
    () => {
        deps!();
        pub const ERROR_NO_UNICODE_TRANSLATION : WIN32_ERROR = 1113u32 ;
    };
}

ERROR_NO_UNICODE_TRANSLATION!();