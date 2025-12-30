// Generated macro for macro_90 (macro)
macro_rules! Depcrate_windows_sysmacro_90 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_90"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymFunctionTableAccess64 (hprocess : HANDLE , addrbase : u64) -> * mut core :: ffi :: c_void) ;
};
}
