// Generated macro for PFUNCTION_TABLE_ACCESS_ROUTINE64 (type)
macro_rules! Depcrate_windows_sysPFUNCTION_TABLE_ACCESS_ROUTINE64 {
() => {
// Module: crate::windows_sys
// Provides: {"PFUNCTION_TABLE_ACCESS_ROUTINE64"}
// Dependencies: {}
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option < unsafe extern "system" fn (ahprocess : HANDLE , addrbase : u64) -> * mut core :: ffi :: c_void > ;
};
}
