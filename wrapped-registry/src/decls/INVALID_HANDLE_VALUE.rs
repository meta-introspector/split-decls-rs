macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! INVALID_HANDLE_VALUE {
    () => {
        deps!();
        pub const INVALID_HANDLE_VALUE : HANDLE = - 1i32 as _ ;
    };
}

INVALID_HANDLE_VALUE!();