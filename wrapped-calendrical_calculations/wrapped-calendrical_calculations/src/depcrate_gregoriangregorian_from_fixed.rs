// Generated macro for gregorian_from_fixed (function)
macro_rules! Depcrate_gregoriangregorian_from_fixed {
() => {
// Module: crate::gregorian
// Provides: {"gregorian_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1525-L1540>"] pub fn gregorian_from_fixed (date : RataDie) -> Result < (i32 , u8 , u8) , I32CastError > { let year = year_from_fixed (date) ? ; let day_of_year = date - day_before_year (year) ; let (month , day) = year_day (year , day_of_year as u16) ; Ok ((year , month , day)) }
};
}
