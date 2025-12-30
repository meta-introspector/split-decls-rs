// Generated macro for RuleOnP (enum)
macro_rules! Depcrate_tz_zicRuleOnP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleOnP"}
// Dependencies: {}
# [doc = " The day of the month in which a rule becomes active."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum RuleOnP { # [doc = " A specific fixed day of a month."] Day { day : t :: Day } , # [doc = " The last weekday of a month."] Last { weekday : Weekday } , # [doc = " The weekday on or before a particular day of the month."] OnOrBefore { weekday : Weekday , day : t :: Day } , # [doc = " The weekday on or after a particular day of the month."] OnOrAfter { weekday : Weekday , day : t :: Day } , }
};
}
