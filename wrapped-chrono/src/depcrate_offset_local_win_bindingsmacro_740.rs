// Generated macro for macro_740 (macro)
macro_rules! Depcrate_offset_local_win_bindingsmacro_740 {
() => {
// Module: crate::offset::local::win_bindings
// Provides: {"macro_740"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn SystemTimeToFileTime (lpsystemtime : * const SYSTEMTIME , lpfiletime : * mut FILETIME) -> BOOL) ;
};
}
