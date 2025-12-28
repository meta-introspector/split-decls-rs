macro_rules! deps {
    () => {
        HRESULT!();
        IUnknown_Vtbl!();
    };
}

macro_rules! IWeakReferenceSource_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct IWeakReferenceSource_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetWeakReference : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
    };
}

IWeakReferenceSource_Vtbl!()