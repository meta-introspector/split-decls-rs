// Generated macro for in_leap_year (function)
macro_rules! Depcrate_parsers_datetimein_leap_year {
() => {
// Module: crate::parsers::datetime
// Provides: {"in_leap_year"}
// Dependencies: {}
# [doc = " Utility that returns whether a year is a leap year."] # [inline] fn in_leap_year (year : i32) -> bool { if year % 4 != 0 { false } else if year % 4 == 0 && year % 100 != 0 { true } else if year % 100 == 0 && year % 400 != 0 { false } else { assert_eq ! (year % 400 , 0) ; true } }
};
}
