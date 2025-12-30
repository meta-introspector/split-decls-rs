// Generated macro for days_in_prev_year (function)
macro_rules! Depcrate_chinese_baseddays_in_prev_year {
() => {
// Module: crate::chinese_based
// Provides: {"days_in_prev_year"}
// Dependencies: {}
# [doc = " Given a new year, calculate the number of days in the previous year"] pub fn days_in_prev_year < C : ChineseBased > (new_year : RataDie) -> u16 { let date = new_year - 300 ; let prev_solstice = winter_solstice_on_or_before :: < C > (date) ; let (prev_new_year , _) = new_year_on_or_before_fixed_date :: < C > (date , prev_solstice) ; u16 :: try_from (new_year - prev_new_year) . unwrap_or (360) }
};
}
