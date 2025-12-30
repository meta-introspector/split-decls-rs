// Generated macro for k_day_after (function)
macro_rules! Depcrate_helpersk_day_after {
() => {
// Module: crate::helpers
// Provides: {"k_day_after"}
// Dependencies: {}
# [doc = " returns the weekday (0-6) after (strictly) the fixed date"] pub (crate) const fn k_day_after (weekday : i64 , fixed : RataDie) -> RataDie { let day_of_week = fixed . to_i64_date () . rem_euclid (7) ; let beginning_of_week = fixed . to_i64_date () - day_of_week ; let day = beginning_of_week + weekday ; RataDie :: new (day + if weekday <= day_of_week { 7 } else { 0 }) }
};
}
