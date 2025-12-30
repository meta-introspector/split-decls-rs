// Generated macro for ChangeTime (enum)
macro_rules! Depcrate_lineChangeTime {
() => {
// Module: crate::line
// Provides: {"ChangeTime"}
// Dependencies: {}
# [doc = " The time at which the rules change for a location."] # [doc = ""] # [doc = " This is described with as few units as possible: a change that occurs at"] # [doc = " the beginning of the year lists only the year, a change that occurs on a"] # [doc = " particular day has to list the year, month, and day, and one that occurs"] # [doc = " at a particular second has to list everything."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum ChangeTime { # [doc = " The earliest point in a particular **year**."] UntilYear (Year) , # [doc = " The earliest point in a particular **month**."] UntilMonth (Year , Month) , # [doc = " The earliest point in a particular **day**."] UntilDay (Year , Month , DaySpec) , # [doc = " The earliest point in a particular **hour, minute, or second**."] UntilTime (Year , Month , DaySpec , TimeSpecAndType) , }
};
}
