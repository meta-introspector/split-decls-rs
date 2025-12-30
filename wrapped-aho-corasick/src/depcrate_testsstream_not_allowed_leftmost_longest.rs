// Generated macro for stream_not_allowed_leftmost_longest (function)
macro_rules! Depcrate_testsstream_not_allowed_leftmost_longest {
() => {
// Module: crate::tests
// Provides: {"stream_not_allowed_leftmost_longest"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] # [should_panic] fn stream_not_allowed_leftmost_longest () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostLongest) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . stream_find_iter (& b"" [..]) . count () , 0) ; }
};
}
