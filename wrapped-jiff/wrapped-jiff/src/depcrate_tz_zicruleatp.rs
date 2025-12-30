// Generated macro for RuleAtP (struct)
macro_rules! Depcrate_tz_zicRuleAtP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleAtP"}
// Dependencies: {}
# [doc = " The time of day at which a rule becomes active."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] struct RuleAtP { # [doc = " The amount of time to add to the start of the day specified by"] # [doc = " `RuleOnP`. This may be negative."] span : SpanFieldwise , # [doc = " An optional suffix indicating how to interpret the overall time at"] # [doc = " which a rule takes effect. As I understand it, this applies to the"] # [doc = " entire datetime that is specified by IN, ON and AT and not just the"] # [doc = " AT portion. (Because the suffix may cause the datetime to have an"] # [doc = " offset applied to it, and that offset can change the day!)"] # [doc = ""] # [doc = " When not present, wall clock time is assumed."] suffix : Option < RuleAtSuffixP > , }
};
}
