// Generated macro for IUnknown_Vtbl (struct)
macro_rules! Depcrate_interface_sys_no_coreIUnknown_Vtbl {
() => {
// Module: crate::interface_sys_no_core
// Provides: {"IUnknown_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IUnknown_Vtbl { pub QueryInterface : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , iid : * const GUID , interface : * mut * mut core :: ffi :: c_void ,) -> HRESULT , pub AddRef : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , pub Release : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , }
};
}
