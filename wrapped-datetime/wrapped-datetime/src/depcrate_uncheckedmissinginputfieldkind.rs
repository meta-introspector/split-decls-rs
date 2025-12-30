// Generated macro for MissingInputFieldKind (enum)
macro_rules! Depcrate_uncheckedMissingInputFieldKind {
() => {
// Module: crate::unchecked
// Provides: {"MissingInputFieldKind"}
// Dependencies: {}
# [doc = " The kind of a missing datetime input field."] # [non_exhaustive] # [derive (Debug , PartialEq , Copy , Clone , displaydoc :: Display)] pub enum MissingInputFieldKind { # [doc = " Day of month"] DayOfMonth , # [doc = " Day of year"] DayOfYear , # [doc = " RataDie"] RataDie , # [doc = " Hour"] Hour , # [doc = " Minute"] Minute , # [doc = " Month"] Month , # [doc = " Second"] Second , # [doc = " Subsecond"] Subsecond , # [doc = " Weekday"] Weekday , # [doc = " Year"] Year , # [doc = " Cyclic year"] YearCyclic , # [doc = " Era year"] YearEra , # [doc = " Time zone identifier"] TimeZoneId , # [doc = " Time zone name timestamp"] TimeZoneNameTimestamp , # [doc = " Unused as of 2.1.0"] # [deprecated (since = "2.1.0" , note = "unused, never returned")] TimeZoneVariant , }
};
}
