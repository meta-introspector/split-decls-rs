// Generated macro for SplitRange (enum)
macro_rules! Depcrate_nfa_thompson_range_trieSplitRange {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"SplitRange"}
// Dependencies: {}
# [doc = " A tagged range indicating how it was derived from a pair of ranges."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum SplitRange { Old (Utf8Range) , New (Utf8Range) , Both (Utf8Range) , }
};
}
