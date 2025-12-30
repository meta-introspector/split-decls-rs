// Generated macro for is_leap_year (function)
macro_rules! Depcrate_gregorianis_leap_year {
() => {
// Module: crate::gregorian
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Whether or not `year` is a leap year"] # [doc = ""] # [doc = " Inspired by Neri-Schneider <https://www.youtube.com/watch?v=J9KijLyP-yg&t=1239s>"] pub const fn is_leap_year (year : i32) -> bool { if year % 25 != 0 { year % 4 == 0 } else { year % 16 == 0 } }
};
}
