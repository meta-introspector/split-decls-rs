// Generated macro for IWeakReference_Vtbl (struct)
macro_rules! Depcrate_imp_com_bindingsIWeakReference_Vtbl {
() => {
// Module: crate::imp::com_bindings
// Provides: {"IWeakReference_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IWeakReference_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub Resolve : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
