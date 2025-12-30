// Generated macro for IGuidHelperStatics_Vtbl (struct)
macro_rules! Depcrate_class_staticIGuidHelperStatics_Vtbl {
() => {
// Module: crate::class_static
// Provides: {"IGuidHelperStatics_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IGuidHelperStatics_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub CreateNewGuid : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: GUID ,) -> windows_core :: HRESULT , pub Empty : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: GUID ,) -> windows_core :: HRESULT , pub Equals : unsafe extern "system" fn (* mut core :: ffi :: c_void , & windows_core :: GUID , & windows_core :: GUID , * mut bool ,) -> windows_core :: HRESULT , }
};
}
