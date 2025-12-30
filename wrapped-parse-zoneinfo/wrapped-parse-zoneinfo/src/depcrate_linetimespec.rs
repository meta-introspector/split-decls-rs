// Generated macro for TimeSpec (enum)
macro_rules! Depcrate_lineTimeSpec {
() => {
// Module: crate::line
// Provides: {"TimeSpec"}
// Dependencies: {}
# [doc = " A **time** definition field."] # [doc = ""] # [doc = " A time must have an hours component, with optional minutes and seconds"] # [doc = " components. It can also be negative with a starting ‘-’."] # [doc = ""] # [doc = " Hour 0 is midnight at the start of the day, and Hour 24 is midnight at the"] # [doc = " end of the day."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum TimeSpec { # [doc = " A number of hours."] Hours (i8) , # [doc = " A number of hours and minutes."] HoursMinutes (i8 , i8) , # [doc = " A number of hours, minutes, and seconds."] HoursMinutesSeconds (i8 , i8 , i8) , # [doc = " Zero, or midnight at the start of the day."] Zero , }
};
}
