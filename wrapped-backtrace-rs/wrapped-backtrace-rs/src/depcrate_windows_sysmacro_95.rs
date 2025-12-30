// Generated macro for macro_95 (macro)
macro_rules! Depcrate_windows_sysmacro_95 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_95"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymGetSearchPathW (hprocess : HANDLE , searchpatha : PWSTR , searchpathlength : u32) -> BOOL) ;
};
}
