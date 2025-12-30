// Generated macro for RuleDay (enum)
macro_rules! Depcrate_offset_local_tz_info_ruleRuleDay {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"RuleDay"}
// Dependencies: {}
# [doc = " Transition rule day"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] enum RuleDay { # [doc = " Julian day in `[1, 365]`, without taking occasional Feb 29 into account, which is not referenceable"] Julian1WithoutLeap (u16) , # [doc = " Zero-based Julian day in `[0, 365]`, taking occasional Feb 29 into account"] Julian0WithLeap (u16) , # [doc = " Day represented by a month, a month week and a week day"] MonthWeekday { # [doc = " Month in `[1, 12]`"] month : u8 , # [doc = " Week of the month in `[1, 5]`, with `5` representing the last week of the month"] week : u8 , # [doc = " Day of the week in `[0, 6]` from Sunday"] week_day : u8 , } , }
};
}
