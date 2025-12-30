// Generated macro for macro_741 (macro)
macro_rules! Depcrate_offset_local_win_bindingsmacro_741 {
() => {
// Module: crate::offset::local::win_bindings
// Provides: {"macro_741"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime (lptimezoneinformation : * const TIME_ZONE_INFORMATION , lpuniversaltime : * const SYSTEMTIME , lplocaltime : * mut SYSTEMTIME) -> BOOL) ;
};
}
