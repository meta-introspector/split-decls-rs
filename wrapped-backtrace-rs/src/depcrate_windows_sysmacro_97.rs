// Generated macro for macro_97 (macro)
macro_rules! Depcrate_windows_sysmacro_97 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_97"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymQueryInlineTrace (hprocess : HANDLE , startaddress : u64 , startcontext : u32 , startretaddress : u64 , curaddress : u64 , curcontext : * mut u32 , curframeindex : * mut u32) -> BOOL) ;
};
}
