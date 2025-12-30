// Generated macro for IReference_Vtbl (struct)
macro_rules! Depcrate_bindingsIReference_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IReference_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IReference_Vtbl < T > where T : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub Value : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < T > ,) -> windows_core :: HRESULT , T : core :: marker :: PhantomData < T > , }
};
}
