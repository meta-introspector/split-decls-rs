// Generated macro for impl_38 (impl)
macro_rules! Depcrate_chinese_basedimpl_38 {
() => {
// Module: crate::chinese_based
// Provides: {"impl_38"}
// Dependencies: {}
impl YearBounds { # [doc = " Compute the YearBounds for the lunar year (年) containing `date`,"] # [doc = " as well as the corresponding solar year (歲). Note that since the two"] # [doc = " years overlap significantly but not entirely, the solstice bounds for the solar"] # [doc = " year *may* not include `date`."] # [inline] pub fn compute < C : ChineseBased > (date : RataDie) -> Self { let prev_solstice = winter_solstice_on_or_before :: < C > (date) ; let (new_year , next_solstice) = new_year_on_or_before_fixed_date :: < C > (date , prev_solstice) ; let next_new_year = new_year_on_or_before_fixed_date :: < C > (new_year + 400 , next_solstice) . 0 ; Self { new_year , next_new_year , } } # [doc = " The number of days in this year"] pub fn count_days (self) -> u16 { let result = self . next_new_year - self . new_year ; debug_assert ! (((u16 :: MIN as i64) ..= (u16 :: MAX as i64)) . contains (& result) , "Days in year should be in range of u16.") ; result as u16 } # [doc = " Whether or not this is a leap year"] pub fn is_leap (self) -> bool { let difference = self . next_new_year - self . new_year ; difference > 365 } }
};
}
