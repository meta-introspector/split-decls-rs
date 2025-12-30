// Generated macro for impl_17 (impl)
macro_rules! Depcrate_lineimpl_17 {
() => {
// Module: crate::line
// Provides: {"impl_17"}
// Dependencies: {}
impl FromStr for DaySpec { type Err = Error ; fn from_str (input : & str) -> Result < Self , Self :: Err > { if input . chars () . all (| c | c . is_ascii_digit ()) { return Ok (DaySpec :: Ordinal (input . parse () . unwrap ())) ; } else if let Some (remainder) = input . strip_prefix ("last") { let weekday = remainder . parse () ? ; return Ok (DaySpec :: Last (weekday)) ; } let weekday = match input . get (.. 3) { Some (wd) => Weekday :: from_str (wd) ? , None => return Err (Error :: InvalidDaySpec (input . to_string ())) , } ; let dir = match input . get (3 .. 5) { Some (">=") => true , Some ("<=") => false , _ => return Err (Error :: InvalidDaySpec (input . to_string ())) , } ; let day = match input . get (5 ..) { Some (day) => u8 :: from_str (day) . map_err (| _ | Error :: InvalidDaySpec (input . to_string ())) ? , None => return Err (Error :: InvalidDaySpec (input . to_string ())) , } as i8 ; Ok (match dir { true => DaySpec :: FirstOnOrAfter (weekday , day) , false => DaySpec :: LastOnOrBefore (weekday , day) , }) } }
};
}
