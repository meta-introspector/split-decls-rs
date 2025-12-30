// Generated macro for macro_4 (macro)
macro_rules! Depcrate_bindingsmacro_4 {
() => {
// Module: crate::bindings
// Provides: {"macro_4"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn FormatMessageW (dwflags : FORMAT_MESSAGE_OPTIONS , lpsource : * const core :: ffi :: c_void , dwmessageid : u32 , dwlanguageid : u32 , lpbuffer : PWSTR , nsize : u32 , arguments : * const * const i8) -> u32) ;
};
}
