// Generated macro for HalfMatch (struct)
macro_rules! Depcrate_util_searchHalfMatch {
() => {
// Module: crate::util::search
// Provides: {"HalfMatch"}
// Dependencies: {}
# [doc = " A representation of \"half\" of a match reported by a DFA."] # [doc = ""] # [doc = " This is called a \"half\" match because it only includes the end location (or"] # [doc = " start location for a reverse search) of a match. This corresponds to the"] # [doc = " information that a single DFA scan can report. Getting the other half of"] # [doc = " the match requires a second scan with a reversed DFA."] # [doc = ""] # [doc = " A half match also includes the pattern that matched. The pattern is"] # [doc = " identified by an ID, which corresponds to its position (starting from `0`)"] # [doc = " relative to other patterns used to construct the corresponding DFA. If only"] # [doc = " a single pattern is provided to the DFA, then all matches are guaranteed to"] # [doc = " have a pattern ID of `0`."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct HalfMatch { # [doc = " The pattern ID."] pattern : PatternID , # [doc = " The offset of the match."] # [doc = ""] # [doc = " For forward searches, the offset is exclusive. For reverse searches,"] # [doc = " the offset is inclusive."] offset : usize , }
};
}
