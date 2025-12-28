macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! PGET_MODULE_BASE_ROUTINE64 {
    () => {
        deps!();
        pub type PGET_MODULE_BASE_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , address : u64) -> u64 > ;
    };
}

PGET_MODULE_BASE_ROUTINE64!();