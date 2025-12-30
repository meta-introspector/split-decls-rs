// Generated macro for fixed_from_observational_islamic (function)
macro_rules! Depcrate_islamicfixed_from_observational_islamic {
() => {
// Module: crate::islamic
// Provides: {"fixed_from_observational_islamic"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6904>"] pub fn fixed_from_observational_islamic (year : i32 , month : u8 , day : u8 , location : Location ,) -> RataDie { let year = i64 :: from (year) ; let month = i64 :: from (month) ; let day = i64 :: from (day) ; let midmonth = ISLAMIC_EPOCH_FRIDAY . to_f64_date () + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH ; let lunar_phase = Astronomical :: calculate_new_moon_at_or_before (RataDie :: new (midmonth as i64)) ; Astronomical :: phasis_on_or_before (RataDie :: new (midmonth as i64) , location , Some (lunar_phase)) + day - 1 }
};
}
