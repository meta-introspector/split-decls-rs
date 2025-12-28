macro_rules! deps {
    () => {
        PENUMLOADED_MODULES_CALLBACKW64!();
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_58 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn EnumerateLoadedModulesW64 (hprocess : HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : * const core :: ffi :: c_void) -> BOOL) ;
    };
}

macro_58!()