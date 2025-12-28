macro_rules! deps {
    () => {
        PCWSTR!();
        BOOL!();
    };
}

macro_rules! PENUMLOADED_MODULES_CALLBACKW64 {
    () => {
        deps!();
        pub type PENUMLOADED_MODULES_CALLBACKW64 = Option < unsafe extern "system" fn (modulename : PCWSTR , modulebase : u64 , modulesize : u32 , usercontext : * const core :: ffi :: c_void ,) -> BOOL , > ;
    };
}

PENUMLOADED_MODULES_CALLBACKW64!()