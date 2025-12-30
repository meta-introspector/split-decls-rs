// Generated macro for days_until_month (function)
macro_rules! Depcrate_chinese_baseddays_until_month {
() => {
// Module: crate::chinese_based
// Provides: {"days_until_month"}
// Dependencies: {}
# [doc = " Given the new year and a month/day pair, calculate the number of days until the first day of the given month"] pub fn days_until_month < C : ChineseBased > (new_year : RataDie , month : u8) -> u16 { let month_approx = 28_u16 . saturating_mul (u16 :: from (month) - 1) ; let new_moon = new_moon_on_or_after :: < C > (new_year . as_moment () + (month_approx as f64)) ; let result = new_moon - new_year ; debug_assert ! (((u16 :: MIN as i64) ..= (u16 :: MAX as i64)) . contains (& result) , "Result {result} from new moon: {new_moon:?} and new year: {new_year:?} should be in range of u16!") ; result as u16 }
};
}
