// Generated macro for macro_87 (macro)
macro_rules! Depcrate_windows_sysmacro_87 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_87"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace (hprocess : HANDLE , address : u64) -> u32) ;
};
}
