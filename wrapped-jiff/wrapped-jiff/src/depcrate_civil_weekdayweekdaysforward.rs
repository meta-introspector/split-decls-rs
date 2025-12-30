// Generated macro for WeekdaysForward (struct)
macro_rules! Depcrate_civil_weekdayWeekdaysForward {
() => {
// Module: crate::civil::weekday
// Provides: {"WeekdaysForward"}
// Dependencies: {}
# [doc = " An unending iterator of the days of the week."] # [doc = ""] # [doc = " This iterator is created by calling [`Weekday::cycle_forward`]."] # [derive (Clone , Debug)] pub struct WeekdaysForward { it : core :: iter :: Cycle < core :: array :: IntoIter < Weekday , 7 > > , }
};
}
