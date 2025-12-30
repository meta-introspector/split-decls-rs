// Generated macro for parse_duration (function)
macro_rules! Depcrate_durationparse_duration {
() => {
// Module: crate::duration
// Provides: {"parse_duration"}
// Dependencies: {}
# [doc = " Parse duration object `1hour 12min 5s`"] # [doc = ""] # [doc = " The duration object is a concatenation of time spans. Where each time"] # [doc = " span is an integer number and a suffix. Supported suffixes:"] # [doc = ""] # [doc = " * `nsec`, `ns` -- nanoseconds"] # [doc = " * `usec`, `us`, `µs` -- microseconds"] # [doc = " * `msec`, `ms` -- milliseconds"] # [doc = " * `seconds`, `second`, `sec`, `s`"] # [doc = " * `minutes`, `minute`, `min`, `m`"] # [doc = " * `hours`, `hour`, `hr`, `hrs`, `h`"] # [doc = " * `days`, `day`, `d`"] # [doc = " * `weeks`, `week`, `wk`, `wks`, `w`"] # [doc = " * `months`, `month`, `M` -- defined as 30.44 days"] # [doc = " * `years`, `year`, `yr`, `yrs`, `y` -- defined as 365.25 days"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " use humantime::parse_duration;"] # [doc = ""] # [doc = " assert_eq!(parse_duration(\"2h 37min\"), Ok(Duration::new(9420, 0)));"] # [doc = " assert_eq!(parse_duration(\"32ms\"), Ok(Duration::new(0, 32_000_000)));"] # [doc = " assert_eq!(parse_duration(\"4.2s\"), Ok(Duration::new(4, 200_000_000)));"] # [doc = " ```"] pub fn parse_duration (s : & str) -> Result < Duration , Error > { if s == "0" { return Ok (Duration :: ZERO) ; } Parser { iter : s . chars () , src : s , } . parse () }
};
}
