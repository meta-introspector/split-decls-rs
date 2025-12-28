macro_rules! deps {
    () => {
        GUID!();
        HRESULT!();
        IUnknown_Vtbl!();
    };
}

macro_rules! IWeakReference_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct IWeakReference_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub Resolve : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
    };
}

IWeakReference_Vtbl!()