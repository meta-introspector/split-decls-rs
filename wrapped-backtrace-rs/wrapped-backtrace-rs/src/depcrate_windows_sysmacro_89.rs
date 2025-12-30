// Generated macro for macro_89 (macro)
macro_rules! Depcrate_windows_sysmacro_89 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_89"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymFromInlineContextW (hprocess : HANDLE , address : u64 , inlinecontext : u32 , displacement : * mut u64 , symbol : * mut SYMBOL_INFOW) -> BOOL) ;
};
}
