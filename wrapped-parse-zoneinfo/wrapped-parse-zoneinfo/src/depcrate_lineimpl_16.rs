// Generated macro for impl_16 (impl)
macro_rules! Depcrate_lineimpl_16 {
() => {
// Module: crate::line
// Provides: {"impl_16"}
// Dependencies: {}
impl DaySpec { # [doc = " Converts this day specification to a concrete date, given the year and"] # [doc = " month it should occur in."] pub fn to_concrete_day (& self , year : i64 , month : Month) -> (Month , i8) { let leap = is_leap (year) ; let length = month . length (leap) ; let prev_length = month . prev_in_year () . map (| m | m . length (leap)) . unwrap_or (0) ; match * self { DaySpec :: Ordinal (day) => (month , day) , DaySpec :: Last (weekday) => (month , (1 .. length + 1) . rev () . find (| & day | Weekday :: calculate (year , month , day) == weekday) . unwrap () ,) , DaySpec :: LastOnOrBefore (weekday , day) => (- 7 .. day + 1) . rev () . flat_map (| inner_day | { if inner_day >= 1 && Weekday :: calculate (year , month , inner_day) == weekday { Some ((month , inner_day)) } else if inner_day < 1 && Weekday :: calculate (year , month . prev_in_year () . unwrap () , prev_length + inner_day ,) == weekday { Some ((month . prev_in_year () . unwrap () , prev_length + inner_day)) } else { None } }) . next () . unwrap () , DaySpec :: FirstOnOrAfter (weekday , day) => (day .. day + 8) . flat_map (| inner_day | { if inner_day <= length && Weekday :: calculate (year , month , inner_day) == weekday { Some ((month , inner_day)) } else if inner_day > length && Weekday :: calculate (year , month . next_in_year () . unwrap () , inner_day - length ,) == weekday { Some ((month . next_in_year () . unwrap () , inner_day - length)) } else { None } }) . next () . unwrap () , } } }
};
}
