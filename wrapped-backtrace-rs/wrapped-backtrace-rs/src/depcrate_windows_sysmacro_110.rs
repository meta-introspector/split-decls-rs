// Generated macro for macro_110 (macro)
macro_rules! Depcrate_windows_sysmacro_110 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_110"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn Module32FirstW (hsnapshot : HANDLE , lpme : * mut MODULEENTRY32W) -> BOOL) ;
};
}
