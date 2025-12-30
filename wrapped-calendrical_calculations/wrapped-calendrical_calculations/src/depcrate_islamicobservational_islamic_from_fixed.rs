// Generated macro for observational_islamic_from_fixed (function)
macro_rules! Depcrate_islamicobservational_islamic_from_fixed {
() => {
// Module: crate::islamic
// Provides: {"observational_islamic_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L6983-L6995>"] pub fn observational_islamic_from_fixed (date : RataDie , location : Location) -> (i32 , u8 , u8) { let lunar_phase = Astronomical :: calculate_new_moon_at_or_before (date) ; let crescent = Astronomical :: phasis_on_or_before (date , location , Some (lunar_phase)) ; let elapsed_months = ((crescent - ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH) . round () as i32 ; let year = elapsed_months . div_euclid (12) + 1 ; let month = elapsed_months . rem_euclid (12) + 1 ; let day = (date - crescent + 1) as u8 ; (year , month as u8 , day) }
};
}
