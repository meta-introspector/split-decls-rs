// Generated macro for macro_96 (macro)
macro_rules! Depcrate_windows_sysmacro_96 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_96"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymInitializeW (hprocess : HANDLE , usersearchpath : PCWSTR , finvadeprocess : BOOL) -> BOOL) ;
};
}
