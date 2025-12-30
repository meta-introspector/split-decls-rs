// Generated macro for saudi_islamic_month_days (function)
macro_rules! Depcrate_islamicsaudi_islamic_month_days {
() => {
// Module: crate::islamic
// Provides: {"saudi_islamic_month_days"}
// Dependencies: {}
# [doc = " The number of days in a month for the Saudi (Umm Al-Qura) calendar"] pub fn saudi_islamic_month_days (year : i32 , month : u8) -> u8 { let midmonth = Moment :: new (ISLAMIC_EPOCH_FRIDAY . to_f64_date () + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH ,) ; let midmonth_next = midmonth + MEAN_SYNODIC_MONTH ; let month_start = saudi_new_month_on_or_before (midmonth . as_rata_die ()) ; let next_month_start = saudi_new_month_on_or_before (midmonth_next . as_rata_die ()) ; let diff = next_month_start - month_start ; debug_assert ! (diff <= 30 || ! WELL_BEHAVED_ASTRONOMICAL_RANGE . contains (& month_start) , "umm-al-qura months must not be more than 30 days") ; u8 :: try_from (diff) . unwrap_or (30) }
};
}
