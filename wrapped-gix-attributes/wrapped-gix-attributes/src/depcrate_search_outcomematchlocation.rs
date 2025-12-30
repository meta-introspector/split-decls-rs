// Generated macro for MatchLocation (struct)
macro_rules! Depcrate_search_outcomeMatchLocation {
() => {
// Module: crate::search::outcome
// Provides: {"MatchLocation"}
// Dependencies: {}
# [doc = " A version of `MatchLocation` without references."] # [derive (Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct MatchLocation { # [doc = " The path to the source from which the pattern was loaded, or `None` if it was specified by other means."] pub source : Option < RefMapKey > , # [doc = " The line at which the pattern was found in its `source` file, or the occurrence in which it was provided."] pub sequence_number : usize , }
};
}
