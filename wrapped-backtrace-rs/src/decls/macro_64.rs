macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! macro_64 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymFunctionTableAccess64 (hprocess : HANDLE , addrbase : u64) -> * mut core :: ffi :: c_void) ;
    };
}

macro_64!()