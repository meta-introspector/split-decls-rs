// Generated macro for tabular_year_from_fixed (function)
macro_rules! Depcrate_islamictabular_year_from_fixed {
() => {
// Module: crate::islamic
// Provides: {"tabular_year_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2090>"] pub fn tabular_year_from_fixed (date : RataDie , epoch : RataDie) -> i32 { i64_to_saturated_i32 ((date - epoch) * 30 / (354 * 30 + 11) + (date >= epoch) as i64) }
};
}
