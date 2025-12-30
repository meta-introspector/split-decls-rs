// Generated macro for macro_32 (macro)
macro_rules! Depcrate_windows_sysmacro_32 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_32"}
// Dependencies: {}
windows_link :: link ! ("ole32.dll" "system" fn CoInitializeEx (pvreserved : * const core :: ffi :: c_void , dwcoinit : u32) -> HRESULT) ;
};
}
