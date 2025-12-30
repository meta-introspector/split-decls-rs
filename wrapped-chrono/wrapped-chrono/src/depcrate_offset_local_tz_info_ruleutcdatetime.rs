// Generated macro for UtcDateTime (struct)
macro_rules! Depcrate_offset_local_tz_info_ruleUtcDateTime {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"UtcDateTime"}
// Dependencies: {}
# [doc = " UTC date time exprimed in the [proleptic gregorian calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar)"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd)] pub (crate) struct UtcDateTime { # [doc = " Year"] pub (crate) year : i32 , # [doc = " Month in `[1, 12]`"] pub (crate) month : u8 , # [doc = " Day of the month in `[1, 31]`"] pub (crate) month_day : u8 , # [doc = " Hours since midnight in `[0, 23]`"] pub (crate) hour : u8 , # [doc = " Minutes in `[0, 59]`"] pub (crate) minute : u8 , # [doc = " Seconds in `[0, 60]`, with a possible leap second"] pub (crate) second : u8 , }
};
}
