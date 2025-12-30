// Generated macro for days_before_month (function)
macro_rules! Depcrate_juliandays_before_month {
() => {
// Module: crate::julian
// Provides: {"days_before_month"}
// Dependencies: {}
# [doc = " The number of days in this year before this month starts"] # [doc = ""] # [doc = " Inspired by Neri-Schneider <https://onlinelibrary.wiley.com/doi/10.1002/spe.3172>"] pub const fn days_before_month (year : i32 , month : u8) -> u16 { if month < 3 { if month == 1 { 0 } else { 31 } } else { 31 + 28 + is_leap_year (year) as u16 + ((979 * (month as u32) - 2919) >> 5) as u16 } }
};
}
