// Generated macro for impl_42 (impl)
macro_rules! Depcrate_shared_posiximpl_42 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_42"}
// Dependencies: {}
impl PosixDay { # [doc = " Convert this date specification to a civil date in the year given."] # [doc = ""] # [doc = " If this date specification couldn't be turned into a date in the year"] # [doc = " given, then `None` is returned. This happens when `366` is given as"] # [doc = " a day, but the year given is not a leap year. In this case, callers may"] # [doc = " want to assume a datetime that is maximal for the year given."] fn to_date (& self , year : i16) -> Option < IDate > { match * self { PosixDay :: JulianOne (day) => { Some (IDate :: from_day_of_year_no_leap (year , day) . expect ("Julian `J day` should be in bounds") ,) } PosixDay :: JulianZero (day) => { IDate :: from_day_of_year (year , day + 1) . ok () } PosixDay :: WeekdayOfMonth { month , week , weekday } => { let weekday = IWeekday :: from_sunday_zero_offset (weekday) ; let first = IDate { year , month , day : 1 } ; let week = if week == 5 { - 1 } else { week } ; debug_assert ! (week == - 1 || (1 ..= 4) . contains (& week)) ; Some (first . nth_weekday_of_month (week , weekday) . expect ("nth weekday always exists") ,) } } } }
};
}
