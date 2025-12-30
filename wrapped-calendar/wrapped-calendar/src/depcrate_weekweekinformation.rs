// Generated macro for WeekInformation (struct)
macro_rules! Depcrate_weekWeekInformation {
() => {
// Module: crate::week
// Provides: {"WeekInformation"}
// Dependencies: {}
# [doc = " Information about the first day of the week and the weekend."] # [derive (Clone , Copy , Debug)] # [non_exhaustive] pub struct WeekInformation { # [doc = " The first day of a week."] pub first_weekday : Weekday , # [doc = " The set of weekend days"] pub weekend : WeekdaySet , }
};
}
