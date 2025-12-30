// Generated macro for impl_186 (impl)
macro_rules! Depcrate_cal_hebrewimpl_186 {
() => {
// Module: crate::cal::hebrew
// Provides: {"impl_186"}
// Dependencies: {}
impl HebrewYear { # [doc = " Convenience method to compute for a given year. Don't use this if you actually need"] # [doc = " a YearInfo that you want to call .new_year() on."] fn compute (value : i32) -> Self { Self { keviyah : YearInfo :: compute_for (value) . keviyah , value , } } fn for_rd (rd : RataDie) -> Self { let (year , value) = YearInfo :: year_containing_rd (rd) ; Self { keviyah : year . keviyah , value , } } fn new_year (self) -> RataDie { self . keviyah . year_info (self . value) . new_year () } }
};
}
