// Generated macro for RuleToP (enum)
macro_rules! Depcrate_tz_zicRuleToP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleToP"}
// Dependencies: {}
# [doc = " The year at which this rule ends (inclusive)."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum RuleToP { # [doc = " The indefinite future."] Max , # [doc = " Repeat the year given in the FROM field."] Only , # [doc = " A specific year at which the rules ends. The year is an inclusive"] # [doc = " bound, but must be greater than or equal to the year in the FROM"] # [doc = " field of the rule."] Year { year : t :: Year } , }
};
}
