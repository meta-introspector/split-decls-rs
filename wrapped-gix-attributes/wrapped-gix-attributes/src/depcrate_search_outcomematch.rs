// Generated macro for Match (struct)
macro_rules! Depcrate_search_outcomeMatch {
() => {
// Module: crate::search::outcome
// Provides: {"Match"}
// Dependencies: {}
# [doc = " A version of `Match` without references."] # [derive (Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct Match { # [doc = " The glob pattern itself, like `/target/*`."] pub pattern : RefMapKey , # [doc = " The key=value pair of the attribute that matched at the pattern. There can be multiple matches per pattern."] pub assignment : RefMapKey , # [doc = " Additional information about the kind of match."] pub kind : MatchKind , # [doc = " Information about the location of the match."] pub location : MatchLocation , }
};
}
