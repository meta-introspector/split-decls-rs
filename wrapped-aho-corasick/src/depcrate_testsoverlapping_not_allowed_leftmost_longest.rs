// Generated macro for overlapping_not_allowed_leftmost_longest (function)
macro_rules! Depcrate_testsoverlapping_not_allowed_leftmost_longest {
() => {
// Module: crate::tests
// Provides: {"overlapping_not_allowed_leftmost_longest"}
// Dependencies: {}
# [test] # [should_panic] fn overlapping_not_allowed_leftmost_longest () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostLongest) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . find_overlapping_iter ("") . count () , 0) ; }
};
}
