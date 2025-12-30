// Generated macro for macro_88 (macro)
macro_rules! Depcrate_windows_sysmacro_88 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_88"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymFromAddrW (hprocess : HANDLE , address : u64 , displacement : * mut u64 , symbol : * mut SYMBOL_INFOW) -> BOOL) ;
};
}
