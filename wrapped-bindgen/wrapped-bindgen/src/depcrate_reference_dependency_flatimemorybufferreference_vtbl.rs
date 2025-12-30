// Generated macro for IMemoryBufferReference_Vtbl (struct)
macro_rules! Depcrate_reference_dependency_flatIMemoryBufferReference_Vtbl {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"IMemoryBufferReference_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IMemoryBufferReference_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Capacity : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , Closed : usize , pub RemoveClosed : unsafe extern "system" fn (* mut core :: ffi :: c_void , i64) -> windows_core :: HRESULT , }
};
}
