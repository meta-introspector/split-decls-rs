// Generated macro for RuleSaveSuffixP (enum)
macro_rules! Depcrate_tz_zicRuleSaveSuffixP {
() => {
// Module: crate::tz::zic
// Provides: {"RuleSaveSuffixP"}
// Dependencies: {}
# [doc = " The optional suffix for the `SAVE` field of a `Rule` line."] # [doc = ""] # [doc = " The default for this depends on the time duration. When it's `0`, the"] # [doc = " default suffix is `Standard`. Otherwise, it's `Dst`."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum RuleSaveSuffixP { # [doc = " The resulting time after applying the corresponding rule should be"] # [doc = " treated as standard time."] Standard , # [doc = " The resulting time after applying the corresponding rule should be"] # [doc = " treated as DST time."] Dst , }
};
}
