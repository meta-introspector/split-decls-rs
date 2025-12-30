// Generated macro for Match (struct)
macro_rules! Depcrate_searchMatch {
() => {
// Module: crate::search
// Provides: {"Match"}
// Dependencies: {}
# [doc = " Describes a matching pattern within a search for ignored paths."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Match < 'a > { # [doc = " The matching search specification, which contains the pathspec as well."] pub pattern : & 'a Pattern , # [doc = " The number of the sequence the matching pathspec was in, or the line of pathspec file it was read from if [Search::source] is not `None`."] pub sequence_number : usize , # [doc = " How the pattern matched."] pub kind : MatchKind , }
};
}
