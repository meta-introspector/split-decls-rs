// Generated macro for fixed_from_saudi_islamic (function)
macro_rules! Depcrate_islamicfixed_from_saudi_islamic {
() => {
// Module: crate::islamic
// Provides: {"fixed_from_saudi_islamic"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6981>"] pub fn fixed_from_saudi_islamic (year : i32 , month : u8 , day : u8) -> RataDie { let midmonth = RataDie :: new (ISLAMIC_EPOCH_FRIDAY . to_i64_date () + (((year as f64 - 1.0) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH) . floor () as i64 ,) ; let first_day_of_month = saudi_new_month_on_or_before (midmonth) . to_i64_date () ; RataDie :: new (first_day_of_month + day as i64 - 1) }
};
}
