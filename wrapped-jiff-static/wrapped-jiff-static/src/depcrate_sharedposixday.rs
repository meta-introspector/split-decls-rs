// Generated macro for PosixDay (enum)
macro_rules! Depcrate_sharedPosixDay {
() => {
// Module: crate::shared
// Provides: {"PosixDay"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum PosixDay { # [doc = " Julian day in a year, no counting for leap days."] # [doc = ""] # [doc = " Valid range is `1..=365`."] JulianOne (i16) , # [doc = " Julian day in a year, counting for leap days."] # [doc = ""] # [doc = " Valid range is `0..=365`."] JulianZero (i16) , # [doc = " The nth weekday of a month."] WeekdayOfMonth { # [doc = " The month."] # [doc = ""] # [doc = " Valid range is: `1..=12`."] month : i8 , # [doc = " The week."] # [doc = ""] # [doc = " Valid range is `1..=5`."] # [doc = ""] # [doc = " One interesting thing to note here (or my interpretation anyway),"] # [doc = " is that a week of `4` means the \"4th weekday in a month\" where as"] # [doc = " a week of `5` means the \"last weekday in a month, even if it's the"] # [doc = " 4th weekday.\""] week : i8 , # [doc = " The weekday."] # [doc = ""] # [doc = " Valid range is `0..=6`, with `0` corresponding to Sunday."] weekday : i8 , } , }
};
}
