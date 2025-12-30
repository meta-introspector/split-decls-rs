// Generated macro for macro_91 (macro)
macro_rules! Depcrate_windows_sysmacro_91 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_91"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymGetLineFromAddrW64 (hprocess : HANDLE , dwaddr : u64 , pdwdisplacement : * mut u32 , line : * mut IMAGEHLP_LINEW64) -> BOOL) ;
};
}
