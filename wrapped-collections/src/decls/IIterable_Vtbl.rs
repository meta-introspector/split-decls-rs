macro_rules! IIterable_Vtbl {
    () => {
        # [repr (C)] # [doc (hidden)] pub struct IIterable_Vtbl < T > where T : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub First : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , T : core :: marker :: PhantomData < T > , }
    };
}

IIterable_Vtbl!()