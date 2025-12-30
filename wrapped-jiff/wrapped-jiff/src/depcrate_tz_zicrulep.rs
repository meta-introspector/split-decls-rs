// Generated macro for RuleP (struct)
macro_rules! Depcrate_tz_zicRuleP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleP"}
// Dependencies: {}
# [doc = " A rule that determines when a particular amount of time should be added to"] # [doc = " a zone's standard time."] # [derive (Clone , Debug , Eq , PartialEq)] struct RuleP { # [doc = " The name of this rule. The name can be referenced by zero or more"] # [doc = " zones in their RULES field. It is guaranteed to be non-empty."] name : RuleNameP , # [doc = " The year at which this rule begins, inclusive."] from : RuleFromP , # [doc = " The year at which this rule ends, inclusive."] to : RuleToP , # [doc = " The month at which this rule becomes active."] inn : RuleInP , # [doc = " The day of the month at which this rule becomes active."] on : RuleOnP , # [doc = " The time of day at which this rule becomes active."] at : RuleAtP , # [doc = " The amount of time to add to standard time when this rule is active."] save : RuleSaveP , # [doc = " The latters that make up the variable part of a zone's abbreviation."] letters : RuleLettersP , }
};
}
