macro_rules! IAsyncOperationWithProgress_Vtbl {
    () => {
        # [repr (C)] # [doc (hidden)] pub struct IAsyncOperationWithProgress_Vtbl < TResult , TProgress > where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub SetProgress : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Progress : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Completed : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < TResult > ,) -> windows_core :: HRESULT , TResult : core :: marker :: PhantomData < TResult > , TProgress : core :: marker :: PhantomData < TProgress > , }
    };
}

IAsyncOperationWithProgress_Vtbl!();