// Generated macro for IAgileReference_Vtbl (struct)
macro_rules! Depcrate_imp_com_bindingsIAgileReference_Vtbl {
() => {
// Module: crate::imp::com_bindings
// Provides: {"IAgileReference_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAgileReference_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub Resolve : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
