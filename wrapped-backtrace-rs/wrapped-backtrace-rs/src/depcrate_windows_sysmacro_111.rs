// Generated macro for macro_111 (macro)
macro_rules! Depcrate_windows_sysmacro_111 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_111"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn Module32NextW (hsnapshot : HANDLE , lpme : * mut MODULEENTRY32W) -> BOOL) ;
};
}
