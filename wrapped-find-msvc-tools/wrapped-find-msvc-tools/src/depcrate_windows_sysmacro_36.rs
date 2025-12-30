// Generated macro for macro_36 (macro)
macro_rules! Depcrate_windows_sysmacro_36 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_36"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryA (lplibfilename : PCSTR) -> HMODULE) ;
};
}
