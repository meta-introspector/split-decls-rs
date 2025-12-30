// Generated macro for macro_8 (macro)
macro_rules! Depcrate_bindingsmacro_8 {
() => {
// Module: crate::bindings
// Provides: {"macro_8"}
// Dependencies: {}
windows_link :: link ! ("ktmw32.dll" "system" fn CreateTransaction (lptransactionattributes : * mut SECURITY_ATTRIBUTES , uow : * mut GUID , createoptions : u32 , isolationlevel : u32 , isolationflags : u32 , timeout : u32 , description : PCWSTR) -> HANDLE) ;
};
}
