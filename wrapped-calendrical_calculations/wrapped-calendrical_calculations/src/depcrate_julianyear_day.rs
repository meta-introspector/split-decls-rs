// Generated macro for year_day (function)
macro_rules! Depcrate_julianyear_day {
() => {
// Module: crate::julian
// Provides: {"year_day"}
// Dependencies: {}
# [doc = " Calculates the month/day from the 1-based day of the year"] pub fn year_day (year : i32 , day_of_year : u16) -> (u8 , u8) { let correction = if day_of_year < 31 + 28 + is_leap_year (year) as u16 { - 1 } else { (! is_leap_year (year)) as i32 } ; let month = ((12 * (day_of_year as i32 + correction) + 373) / 367) as u8 ; let day = (day_of_year - days_before_month (year , month)) as u8 ; (month , day) }
};
}
