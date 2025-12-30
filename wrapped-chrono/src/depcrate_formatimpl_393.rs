// Generated macro for impl_393 (impl)
macro_rules! Depcrate_formatimpl_393 {
() => {
// Module: crate::format
// Provides: {"impl_393"}
// Dependencies: {}
# [doc = " Parsing a `str` into a `Weekday` uses the format [`%A`](./format/strftime/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::Weekday;"] # [doc = ""] # [doc = " assert_eq!(\"Sunday\".parse::<Weekday>(), Ok(Weekday::Sun));"] # [doc = " assert!(\"any day\".parse::<Weekday>().is_err());"] # [doc = " ```"] # [doc = ""] # [doc = " The parsing is case-insensitive."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::Weekday;"] # [doc = " assert_eq!(\"mON\".parse::<Weekday>(), Ok(Weekday::Mon));"] # [doc = " ```"] # [doc = ""] # [doc = " Only the shortest form (e.g. `sun`) and the longest form (e.g. `sunday`) is accepted."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::Weekday;"] # [doc = " assert!(\"thurs\".parse::<Weekday>().is_err());"] # [doc = " ```"] impl FromStr for Weekday { type Err = ParseWeekdayError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if let Ok (("" , w)) = scan :: short_or_long_weekday (s) { Ok (w) } else { Err (ParseWeekdayError { _dummy : () }) } } }
};
}
