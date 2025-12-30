// Generated macro for macro_739 (macro)
macro_rules! Depcrate_offset_local_win_bindingsmacro_739 {
() => {
// Module: crate::offset::local::win_bindings
// Provides: {"macro_739"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn GetTimeZoneInformationForYear (wyear : u16 , pdtzi : * const DYNAMIC_TIME_ZONE_INFORMATION , ptzi : * mut TIME_ZONE_INFORMATION) -> BOOL) ;
};
}
