// Generated macro for IVectorView_Vtbl (struct)
macro_rules! Depcrate_bindingsIVectorView_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IVectorView_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IVectorView_Vtbl < T > where T : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub GetAt : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , * mut windows_core :: AbiType < T > ,) -> windows_core :: HRESULT , pub Size : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , pub IndexOf : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: AbiType < T > , * mut u32 , * mut bool ,) -> windows_core :: HRESULT , pub GetMany : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , u32 , * mut T , * mut u32 ,) -> windows_core :: HRESULT , T : core :: marker :: PhantomData < T > , }
};
}
