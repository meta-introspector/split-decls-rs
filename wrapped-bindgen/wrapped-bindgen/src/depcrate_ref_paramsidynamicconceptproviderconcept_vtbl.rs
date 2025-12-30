// Generated macro for IDynamicConceptProviderConcept_Vtbl (struct)
macro_rules! Depcrate_ref_paramsIDynamicConceptProviderConcept_Vtbl {
() => {
// Module: crate::ref_params
// Provides: {"IDynamicConceptProviderConcept_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IDynamicConceptProviderConcept_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetConcept : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void , * mut bool ,) -> windows_core :: HRESULT , pub SetConcept : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * const windows_core :: GUID , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub NotifyParent : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub NotifyParentChange : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub NotifyDestruct : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
