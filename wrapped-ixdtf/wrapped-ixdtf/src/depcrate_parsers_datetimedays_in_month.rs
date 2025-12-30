// Generated macro for days_in_month (function)
macro_rules! Depcrate_parsers_datetimedays_in_month {
() => {
// Module: crate::parsers::datetime
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Utilty to return the days in month, returns None if month is invalid"] # [inline] fn days_in_month (year : i32 , month : u8) -> Option < u8 > { match month { 1 | 3 | 5 | 7 | 8 | 10 | 12 => Some (31) , 4 | 6 | 9 | 11 => Some (30) , 2 => Some (28 + u8 :: from (in_leap_year (year))) , _ => None , } }
};
}
