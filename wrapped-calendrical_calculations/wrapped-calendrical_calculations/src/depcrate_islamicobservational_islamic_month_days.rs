// Generated macro for observational_islamic_month_days (function)
macro_rules! Depcrate_islamicobservational_islamic_month_days {
() => {
// Module: crate::islamic
// Provides: {"observational_islamic_month_days"}
// Dependencies: {}
# [doc = " The number of days in a month for the observational islamic calendar"] pub fn observational_islamic_month_days (year : i32 , month : u8 , location : Location) -> u8 { let midmonth = ISLAMIC_EPOCH_FRIDAY . to_f64_date () + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH ; let lunar_phase : f64 = Astronomical :: calculate_new_moon_at_or_before (RataDie :: new (midmonth as i64)) ; let f_date = Astronomical :: phasis_on_or_before (RataDie :: new (midmonth as i64) , location , Some (lunar_phase) ,) ; Astronomical :: month_length (f_date , location) }
};
}
