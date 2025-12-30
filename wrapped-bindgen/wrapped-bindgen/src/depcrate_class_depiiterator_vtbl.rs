// Generated macro for IIterator_Vtbl (struct)
macro_rules! Depcrate_class_depIIterator_Vtbl {
() => {
// Module: crate::class_dep
// Provides: {"IIterator_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IIterator_Vtbl < T > where T : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub Current : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < T > ,) -> windows_core :: HRESULT , pub HasCurrent : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut bool) -> windows_core :: HRESULT , pub MoveNext : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut bool) -> windows_core :: HRESULT , pub GetMany : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , * mut T , * mut u32 ,) -> windows_core :: HRESULT , T : core :: marker :: PhantomData < T > , }
};
}
