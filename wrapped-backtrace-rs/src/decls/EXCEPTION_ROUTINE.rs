macro_rules! deps {
    () => {
        EXCEPTION_RECORD!();
        EXCEPTION_DISPOSITION!();
        CONTEXT!();
    };
}

macro_rules! EXCEPTION_ROUTINE {
    () => {
        deps!();
        pub type EXCEPTION_ROUTINE = Option < unsafe extern "system" fn (exceptionrecord : * mut EXCEPTION_RECORD , establisherframe : * const core :: ffi :: c_void , contextrecord : * mut CONTEXT , dispatchercontext : * const core :: ffi :: c_void ,) -> EXCEPTION_DISPOSITION , > ;
    };
}

EXCEPTION_ROUTINE!();