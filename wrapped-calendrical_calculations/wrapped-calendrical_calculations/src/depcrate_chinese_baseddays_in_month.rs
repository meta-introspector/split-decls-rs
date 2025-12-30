// Generated macro for days_in_month (function)
macro_rules! Depcrate_chinese_baseddays_in_month {
() => {
// Module: crate::chinese_based
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Returns the number of days in the given `month` after the given `new_year`."] # [doc = " Also returns the RataDie of the new moon beginning the next month."] pub fn days_in_month < C : ChineseBased > (month : u8 , new_year : RataDie , prev_new_moon : Option < RataDie > ,) -> (u8 , RataDie) { let approx = new_year + ((month - 1) as i64 * 29) ; let prev_new_moon = if let Some (prev_moon) = prev_new_moon { prev_moon } else { new_moon_before :: < C > ((approx + 15) . as_moment ()) } ; let next_new_moon = new_moon_on_or_after :: < C > ((approx + 15) . as_moment ()) ; let result = (next_new_moon - prev_new_moon) as u8 ; debug_assert ! (result == 29 || result == 30 || ! WELL_BEHAVED_ASTRONOMICAL_RANGE . contains (& new_year)) ; (result , next_new_moon) }
};
}
