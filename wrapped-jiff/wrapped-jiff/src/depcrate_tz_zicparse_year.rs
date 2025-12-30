// Generated macro for parse_year (function)
macro_rules! Depcrate_tz_zicparse_year {
() => {
// Module: crate::tz::zic
// Provides: {"parse_year"}
// Dependencies: {}
# [doc = " Parse a signed year value."] # [doc = ""] # [doc = " This ensures the year is within the range supported by Jiff."] fn parse_year (year : & str) -> Result < t :: Year , Error > { let (sign , rest) = if year . starts_with ("-") { (t :: Sign :: N :: < - 1 > () , & year [1 ..]) } else { (t :: Sign :: N :: < 1 > () , year) } ; let number = parse :: i64 (rest . as_bytes ()) . map_err (| e | e . context ("failed to parse year")) ? ; let year = t :: Year :: new (number) . ok_or_else (| | err ! ("year is out of range: {number}")) ? ; Ok (year * sign) }
};
}
