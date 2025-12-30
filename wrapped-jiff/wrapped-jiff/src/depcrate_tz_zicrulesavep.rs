// Generated macro for RuleSaveP (struct)
macro_rules! Depcrate_tz_zicRuleSaveP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleSaveP"}
// Dependencies: {}
# [doc = " The amount of time to add to standard time when the corresponding rule is"] # [doc = " in effect."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] struct RuleSaveP { # [doc = " The amount of time to add. This may be negative."] span : SpanFieldwise , # [doc = " An optional suffix indicating how the resulting time after applying"] # [doc = " this rule should be interpreted. When absent, this defaults to DST."] suffix : Option < RuleSaveSuffixP > , }
};
}
