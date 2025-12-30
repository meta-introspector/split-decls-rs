// Generated macro for macro_742 (macro)
macro_rules! Depcrate_offset_local_win_bindingsmacro_742 {
() => {
// Module: crate::offset::local::win_bindings
// Provides: {"macro_742"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime (lptimezoneinformation : * const TIME_ZONE_INFORMATION , lplocaltime : * const SYSTEMTIME , lpuniversaltime : * mut SYSTEMTIME) -> BOOL) ;
};
}
