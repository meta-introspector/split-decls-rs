// Generated macro for saudi_islamic_from_fixed (function)
macro_rules! Depcrate_islamicsaudi_islamic_from_fixed {
() => {
// Module: crate::islamic
// Provides: {"saudi_islamic_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6996>"] pub fn saudi_islamic_from_fixed (date : RataDie) -> (i32 , u8 , u8) { let crescent = saudi_new_month_on_or_before (date) ; let elapsed_months = ((crescent - ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH) . round () as i64 ; let year = i64_to_saturated_i32 (elapsed_months . div_euclid (12) + 1) ; let month = (elapsed_months . rem_euclid (12) + 1) as u8 ; let day = ((date - crescent) + 1) as u8 ; (year , month , day) }
};
}
