// Generated macro for MetonicCycleType (enum)
macro_rules! Depcrate_hebrew_keviyahMetonicCycleType {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"MetonicCycleType"}
// Dependencies: {}
# [doc = " \"Metonic cycle\" in general refers to any 19-year repeating pattern used by lunisolar"] # [doc = " calendars. The Hebrew calendar uses one where years 3, 6, 8, 11, 14, 17, 19"] # [doc = " are leap years."] # [doc = ""] # [doc = " The Hebrew calendar further categorizes regular years as whether they come before/after/or"] # [doc = " between leap years, and this is used when performing lookups."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] enum MetonicCycleType { # [doc = " Before a leap year (2, 5, 10, 13, 16)"] LMinusOne , # [doc = " After a leap year (1, 4, 9, 12, 15)"] LPlusOne , # [doc = " Between leap years (7. 18)"] LPlusMinusOne , # [doc = " Leap year (3, 6, 8, 11, 14, 17, 19)"] Leap , }
};
}
