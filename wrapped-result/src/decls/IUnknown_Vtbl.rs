macro_rules! deps {
    () => {
        GUID!();
        HRESULT!();
    };
}

macro_rules! IUnknown_Vtbl {
    () => {
        deps!();
        # [repr (C)] pub struct IUnknown_Vtbl { pub QueryInterface : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , iid : * const GUID , interface : * mut * mut core :: ffi :: c_void ,) -> HRESULT , pub AddRef : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , pub Release : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , }
    };
}

IUnknown_Vtbl!()