// Generated macro for macro_92 (macro)
macro_rules! Depcrate_windows_sysmacro_92 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_92"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymGetLineFromInlineContextW (hprocess : HANDLE , dwaddr : u64 , inlinecontext : u32 , qwmodulebaseaddress : u64 , pdwdisplacement : * mut u32 , line : * mut IMAGEHLP_LINEW64) -> BOOL) ;
};
}
