macro_rules! AsyncActionProgressHandler_Vtbl {
    () => {
        # [repr (C)] # [doc (hidden)] pub struct AsyncActionProgressHandler_Vtbl < TProgress > where TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , progressinfo : windows_core :: AbiType < TProgress > ,) -> windows_core :: HRESULT , TProgress : core :: marker :: PhantomData < TProgress > , }
    };
}

AsyncActionProgressHandler_Vtbl!()