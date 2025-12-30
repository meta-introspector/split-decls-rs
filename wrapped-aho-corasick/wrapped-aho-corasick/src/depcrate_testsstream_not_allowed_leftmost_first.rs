// Generated macro for stream_not_allowed_leftmost_first (function)
macro_rules! Depcrate_testsstream_not_allowed_leftmost_first {
() => {
// Module: crate::tests
// Provides: {"stream_not_allowed_leftmost_first"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] # [should_panic] fn stream_not_allowed_leftmost_first () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostFirst) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . stream_find_iter (& b"" [..]) . count () , 0) ; }
};
}
