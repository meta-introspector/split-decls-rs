// Generated macro for SECURITY_ATTRIBUTES (struct)
macro_rules! Depcrate_windows_sysSECURITY_ATTRIBUTES {
() => {
// Module: crate::windows_sys
// Provides: {"SECURITY_ATTRIBUTES"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct SECURITY_ATTRIBUTES { pub nLength : u32 , pub lpSecurityDescriptor : * mut core :: ffi :: c_void , pub bInheritHandle : BOOL , }
};
}
