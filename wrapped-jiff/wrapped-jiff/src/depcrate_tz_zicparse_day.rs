// Generated macro for parse_day (function)
macro_rules! Depcrate_tz_zicparse_day {
() => {
// Module: crate::tz::zic
// Provides: {"parse_day"}
// Dependencies: {}
# [doc = " Parses a day of the month."] # [doc = ""] # [doc = " This checks that the day is in the range 1-31, but otherwise doesn't"] # [doc = " check that it is valid for a particular month."] fn parse_day (string : & str) -> Result < t :: Day , Error > { let number = parse :: i64 (string . as_bytes ()) . map_err (| e | e . context ("failed to parse number for day")) ? ; let day = t :: Day :: new (number) . ok_or_else (| | err ! ("{number} is not a valid day")) ? ; Ok (day) }
};
}
