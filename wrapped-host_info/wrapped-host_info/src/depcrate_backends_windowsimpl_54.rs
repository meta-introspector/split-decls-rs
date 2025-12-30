// Generated macro for impl_54 (impl)
macro_rules! Depcrate_backends_windowsimpl_54 {
() => {
// Module: crate::backends::windows
// Provides: {"impl_54"}
// Dependencies: {}
impl RawHostInfoBackend for WindowsHostInfoBackend { fn raw_region () -> Result < Option < String > , HostInfoError > { let region = windows :: System :: UserProfile :: GlobalizationPreferences :: HomeGeographicRegion () ? ; let s = region . to_string_lossy () ; if s . is_empty () { Ok (None) } else { Ok (Some (s)) } } fn raw_requested_locales () -> Result < Vec < String > , HostInfoError > { let locale = windows :: System :: UserProfile :: GlobalizationPreferences :: Languages () ? ; let len = locale . Size () ? ; let mut locale_vec_str : Vec < String > = Vec :: with_capacity (len as usize) ; for i in 0 .. len { let hstring = locale . GetAt (i) ? ; let string = hstring . to_string_lossy () ; locale_vec_str . push (string) ; } Ok (locale_vec_str) } fn raw_calendar () -> Result < Option < String > , HostInfoError > { let calendar = :: windows :: Globalization :: Calendar :: new () ? ; let system_calendar = :: windows :: Globalization :: Calendar :: GetCalendarSystem (& calendar) ? ; let calendar_type : String = system_calendar . to_string () ; Ok (Some (calendar_type)) } fn raw_first_day_of_week () -> Result < Option < String > , HostInfoError > { Ok (match :: windows :: System :: UserProfile :: GlobalizationPreferences :: WeekStartsOn () ? . 0 { 0 => Some ("sun" . to_string ()) , 1 => Some ("mon" . to_string ()) , 2 => Some ("tue" . to_string ()) , 3 => Some ("wed" . to_string ()) , 4 => Some ("thu" . to_string ()) , 5 => Some ("fri" . to_string ()) , 6 => Some ("sat" . to_string ()) , _ => None , } ,) } }
};
}
