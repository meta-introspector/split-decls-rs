// Generated macro for macro_93 (macro)
macro_rules! Depcrate_windows_sysmacro_93 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_93"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymGetModuleBase64 (hprocess : HANDLE , qwaddr : u64) -> u64) ;
};
}
