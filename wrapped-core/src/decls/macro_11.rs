macro_rules! deps {
    () => {
        HSTRING!();
        GUID!();
        HRESULT!();
    };
}

macro_rules! macro_11 {
    () => {
        deps!();
        windows_link :: link ! ("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory (activatableclassid : HSTRING , iid : * const GUID , factory : * mut * mut core :: ffi :: c_void) -> HRESULT) ;
    };
}

macro_11!();