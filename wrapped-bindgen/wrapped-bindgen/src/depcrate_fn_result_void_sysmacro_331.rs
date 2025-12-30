// Generated macro for macro_331 (macro)
macro_rules! Depcrate_fn_result_void_sysmacro_331 {
() => {
// Module: crate::fn_result_void_sys
// Provides: {"macro_331"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn SetComputerNameA (lpcomputername : windows_sys :: core :: PCSTR) -> windows_sys :: core :: BOOL) ;
};
}
