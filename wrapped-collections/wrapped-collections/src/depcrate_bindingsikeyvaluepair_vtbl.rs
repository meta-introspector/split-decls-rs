// Generated macro for IKeyValuePair_Vtbl (struct)
macro_rules! Depcrate_bindingsIKeyValuePair_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IKeyValuePair_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IKeyValuePair_Vtbl < K , V > where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub Key : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < K > ,) -> windows_core :: HRESULT , pub Value : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < V > ,) -> windows_core :: HRESULT , K : core :: marker :: PhantomData < K > , V : core :: marker :: PhantomData < V > , }
};
}
