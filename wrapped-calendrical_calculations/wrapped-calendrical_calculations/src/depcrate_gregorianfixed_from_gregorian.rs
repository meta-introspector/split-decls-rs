// Generated macro for fixed_from_gregorian (function)
macro_rules! Depcrate_gregorianfixed_from_gregorian {
() => {
// Module: crate::gregorian
// Provides: {"fixed_from_gregorian"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1167-L1189>"] pub const fn fixed_from_gregorian (year : i32 , month : u8 , day : u8) -> RataDie { day_before_year (year) . add (days_before_month (year , month) as i64) . add (day as i64) }
};
}
