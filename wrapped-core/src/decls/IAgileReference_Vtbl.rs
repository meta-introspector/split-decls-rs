macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
        GUID!();
        HRESULT!();
    };
}

macro_rules! IAgileReference_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct IAgileReference_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub Resolve : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
    };
}

IAgileReference_Vtbl!();