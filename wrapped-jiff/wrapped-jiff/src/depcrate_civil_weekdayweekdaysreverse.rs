// Generated macro for WeekdaysReverse (struct)
macro_rules! Depcrate_civil_weekdayWeekdaysReverse {
() => {
// Module: crate::civil::weekday
// Provides: {"WeekdaysReverse"}
// Dependencies: {}
# [doc = " An unending iterator of the days of the week in reverse."] # [doc = ""] # [doc = " This iterator is created by calling [`Weekday::cycle_reverse`]."] # [derive (Clone , Debug)] pub struct WeekdaysReverse { it : core :: iter :: Cycle < core :: array :: IntoIter < Weekday , 7 > > , }
};
}
