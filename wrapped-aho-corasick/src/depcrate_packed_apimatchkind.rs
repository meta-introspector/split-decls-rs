// Generated macro for MatchKind (enum)
macro_rules! Depcrate_packed_apiMatchKind {
() => {
// Module: crate::packed::api
// Provides: {"MatchKind"}
// Dependencies: {}
# [doc = " A knob for controlling the match semantics of a packed multiple string"] # [doc = " searcher."] # [doc = ""] # [doc = " This differs from the [`MatchKind`](crate::MatchKind) type in the top-level"] # [doc = " crate module in that it doesn't support \"standard\" match semantics,"] # [doc = " and instead only supports leftmost-first or leftmost-longest. Namely,"] # [doc = " \"standard\" semantics cannot be easily supported by packed searchers."] # [doc = ""] # [doc = " For more information on the distinction between leftmost-first and"] # [doc = " leftmost-longest, see the docs on the top-level `MatchKind` type."] # [doc = ""] # [doc = " Unlike the top-level `MatchKind` type, the default match semantics for this"] # [doc = " type are leftmost-first."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum MatchKind { # [doc = " Use leftmost-first match semantics, which reports leftmost matches."] # [doc = " When there are multiple possible leftmost matches, the match"] # [doc = " corresponding to the pattern that appeared earlier when constructing"] # [doc = " the automaton is reported."] # [doc = ""] # [doc = " This is the default."] LeftmostFirst , # [doc = " Use leftmost-longest match semantics, which reports leftmost matches."] # [doc = " When there are multiple possible leftmost matches, the longest match"] # [doc = " is chosen."] LeftmostLongest , }
};
}
