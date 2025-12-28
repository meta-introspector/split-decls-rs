macro_rules! deps {
    () => {
        HRESULT!();
        GUID!();
        CLSCTX!();
    };
}

macro_rules! macro_23 {
    () => {
        deps!();
        windows_link :: link ! ("ole32.dll" "system" fn CoCreateInstance (rclsid : * const GUID , punkouter : * mut core :: ffi :: c_void , dwclscontext : CLSCTX , riid : * const GUID , ppv : * mut * mut core :: ffi :: c_void) -> HRESULT) ;
    };
}

macro_23!()