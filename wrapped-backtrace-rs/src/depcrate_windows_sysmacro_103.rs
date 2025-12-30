// Generated macro for macro_103 (macro)
macro_rules! Depcrate_windows_sysmacro_103 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_103"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn CreateToolhelp32Snapshot (dwflags : CREATE_TOOLHELP_SNAPSHOT_FLAGS , th32processid : u32) -> HANDLE) ;
};
}
