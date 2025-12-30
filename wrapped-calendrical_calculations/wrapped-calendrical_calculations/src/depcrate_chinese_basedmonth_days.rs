// Generated macro for month_days (function)
macro_rules! Depcrate_chinese_basedmonth_days {
() => {
// Module: crate::chinese_based
// Provides: {"month_days"}
// Dependencies: {}
# [doc = " Returns the number of days in the given (year, month)."] # [doc = ""] # [doc = " In the Chinese calendar, months start at each"] # [doc = " new moon, so this function finds the number of days between the new moon at the beginning of the given"] # [doc = " month and the new moon at the beginning of the next month."] pub fn month_days < C : ChineseBased > (year : i32 , month : u8) -> u8 { let mid_year = fixed_mid_year_from_year :: < C > (year) ; let prev_solstice = winter_solstice_on_or_before :: < C > (mid_year) ; let new_year = new_year_on_or_before_fixed_date :: < C > (mid_year , prev_solstice) . 0 ; days_in_month :: < C > (month , new_year , None) . 0 }
};
}
