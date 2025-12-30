// Generated macro for saudi_new_month_on_or_before (function)
macro_rules! Depcrate_islamicsaudi_new_month_on_or_before {
() => {
// Module: crate::islamic
// Provides: {"saudi_new_month_on_or_before"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6966>"] pub fn saudi_new_month_on_or_before (date : RataDie) -> RataDie { let last_new_moon = (Astronomical :: lunar_phase_at_or_before (0.0 , date . as_moment ())) . inner () . floor () ; let age = date . to_f64_date () - last_new_moon ; let tau = if age <= 3.0 && ! adjusted_saudi_criterion (date) { last_new_moon - 30.0 } else { last_new_moon } ; next (RataDie :: new (tau as i64) , adjusted_saudi_criterion) }
};
}
