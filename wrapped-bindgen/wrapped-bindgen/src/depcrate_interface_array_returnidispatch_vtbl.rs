// Generated macro for IDispatch_Vtbl (struct)
macro_rules! Depcrate_interface_array_returnIDispatch_Vtbl {
() => {
// Module: crate::interface_array_return
// Provides: {"IDispatch_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IDispatch_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetTypeInfoCount : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , GetTypeInfo : usize , pub GetIDsOfNames : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * const windows_core :: PCWSTR , u32 , u32 , * mut i32 ,) -> windows_core :: HRESULT , Invoke : usize , }
};
}
