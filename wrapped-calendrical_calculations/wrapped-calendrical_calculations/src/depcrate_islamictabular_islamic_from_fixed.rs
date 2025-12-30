// Generated macro for tabular_islamic_from_fixed (function)
macro_rules! Depcrate_islamictabular_islamic_from_fixed {
() => {
// Module: crate::islamic
// Provides: {"tabular_islamic_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2090>"] pub fn tabular_islamic_from_fixed (date : RataDie , epoch : RataDie) -> (i32 , u8 , u8) { let year = tabular_year_from_fixed (date , epoch) ; let prior_days = date - fixed_from_tabular_islamic (year , 1 , 1 , epoch) ; debug_assert ! (prior_days >= 0) ; debug_assert ! (prior_days <= 354) ; let month = (((prior_days * 11) + 330) / 325) as u8 ; debug_assert ! (month <= 12) ; let day = (date - fixed_from_tabular_islamic (year , month , 1 , epoch) + 1) as u8 ; (year , month , day) }
};
}
