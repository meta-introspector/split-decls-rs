// Generated macro for impl_21 (impl)
macro_rules! Depcrate_lineimpl_21 {
() => {
// Module: crate::line
// Provides: {"impl_21"}
// Dependencies: {}
impl FromStr for TimeSpec { type Err = Error ; fn from_str (input : & str) -> Result < Self , Self :: Err > { if input == "-" { return Ok (TimeSpec :: Zero) ; } let neg = if input . starts_with ('-') { - 1 } else { 1 } ; let mut state = TimeSpec :: Zero ; for part in input . split (':') { state = match (state , part) { (TimeSpec :: Zero , hour) => TimeSpec :: Hours (i8 :: from_str (hour) . map_err (| _ | Error :: InvalidTimeSpecAndType (input . to_string ())) ? ,) , (TimeSpec :: Hours (hours) , minutes) if minutes . len () == 2 => TimeSpec :: HoursMinutes (hours , i8 :: from_str (minutes) . map_err (| _ | Error :: InvalidTimeSpecAndType (input . to_string ())) ? * neg ,) , (TimeSpec :: HoursMinutes (hours , minutes) , seconds) if seconds . len () == 2 => { TimeSpec :: HoursMinutesSeconds (hours , minutes , i8 :: from_str (seconds) . map_err (| _ | Error :: InvalidTimeSpecAndType (input . to_string ())) ? * neg ,) } _ => return Err (Error :: InvalidTimeSpecAndType (input . to_string ())) , } ; } Ok (state) } }
};
}
