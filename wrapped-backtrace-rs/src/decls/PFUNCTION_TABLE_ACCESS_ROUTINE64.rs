macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! PFUNCTION_TABLE_ACCESS_ROUTINE64 {
    () => {
        deps!();
        pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option < unsafe extern "system" fn (ahprocess : HANDLE , addrbase : u64) -> * mut core :: ffi :: c_void > ;
    };
}

PFUNCTION_TABLE_ACCESS_ROUTINE64!()