// Generated macro for RuleLettersP (struct)
macro_rules! Depcrate_tz_zicRuleLettersP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleLettersP"}
// Dependencies: {}
# [doc = " The latters that make up the variable part of a zone's abbreviation."] # [doc = ""] # [doc = " For example, if a zone's format is `E%sT`, then the letters might be"] # [doc = " `S` or `D` for standard time and DST, respectively."] # [derive (Clone , Debug , Eq , PartialEq)] struct RuleLettersP { # [doc = " The actual value that should be interpolated. It may be absent, in"] # [doc = " which case, the empty string should be substituted."] part : String , }
};
}
