macro_rules! IMapView_Vtbl {
    () => {
        # [repr (C)] # [doc (hidden)] pub struct IMapView_Vtbl < K , V > where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub Lookup : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: AbiType < K > , * mut windows_core :: AbiType < V > ,) -> windows_core :: HRESULT , pub Size : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , pub HasKey : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: AbiType < K > , * mut bool ,) -> windows_core :: HRESULT , pub Split : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , K : core :: marker :: PhantomData < K > , V : core :: marker :: PhantomData < V > , }
    };
}

IMapView_Vtbl!();