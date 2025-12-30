// Generated macro for impl_14 (impl)
macro_rules! Depcrate_lineimpl_14 {
() => {
// Module: crate::line
// Provides: {"impl_14"}
// Dependencies: {}
impl FromStr for Weekday { type Err = Error ; fn from_str (input : & str) -> Result < Weekday , Self :: Err > { Ok (match & * input . to_ascii_lowercase () { "mon" | "monday" => Weekday :: Monday , "tue" | "tuesday" => Weekday :: Tuesday , "wed" | "wednesday" => Weekday :: Wednesday , "thu" | "thursday" => Weekday :: Thursday , "fri" | "friday" => Weekday :: Friday , "sat" | "saturday" => Weekday :: Saturday , "sun" | "sunday" => Weekday :: Sunday , other => return Err (Error :: FailedWeekdayParse (other . to_string ())) , }) } }
};
}
