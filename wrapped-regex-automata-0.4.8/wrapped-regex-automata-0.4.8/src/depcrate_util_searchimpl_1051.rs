// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_util_searchimpl_1051 {
() => {
// Module: crate::util::search
// Provides: {"impl_1051"}
// Dependencies: {}
impl HalfMatch { # [doc = " Create a new half match from a pattern ID and a byte offset."] # [inline] pub fn new (pattern : PatternID , offset : usize) -> HalfMatch { HalfMatch { pattern , offset } } # [doc = " Create a new half match from a pattern ID and a byte offset."] # [doc = ""] # [doc = " This is like [`HalfMatch::new`], but accepts a `usize` instead of a"] # [doc = " [`PatternID`]. This panics if the given `usize` is not representable"] # [doc = " as a `PatternID`."] # [inline] pub fn must (pattern : usize , offset : usize) -> HalfMatch { HalfMatch :: new (PatternID :: new (pattern) . unwrap () , offset) } # [doc = " Returns the ID of the pattern that matched."] # [doc = ""] # [doc = " The ID of a pattern is derived from the position in which it was"] # [doc = " originally inserted into the corresponding DFA. The first pattern has"] # [doc = " identifier `0`, and each subsequent pattern is `1`, `2` and so on."] # [inline] pub fn pattern (& self) -> PatternID { self . pattern } # [doc = " The position of the match."] # [doc = ""] # [doc = " If this match was produced by a forward search, then the offset is"] # [doc = " exclusive. If this match was produced by a reverse search, then the"] # [doc = " offset is inclusive."] # [inline] pub fn offset (& self) -> usize { self . offset } }
};
}
