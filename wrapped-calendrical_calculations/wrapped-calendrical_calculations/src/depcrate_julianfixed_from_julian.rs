// Generated macro for fixed_from_julian (function)
macro_rules! Depcrate_julianfixed_from_julian {
() => {
// Module: crate::julian
// Provides: {"fixed_from_julian"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1689-L1709>"] pub const fn fixed_from_julian (year : i32 , month : u8 , day : u8) -> RataDie { day_before_year (year) . add (days_before_month (year , month) as i64) . add (day as i64) }
};
}
