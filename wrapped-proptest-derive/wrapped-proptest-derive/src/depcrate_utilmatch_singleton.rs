// Generated macro for match_singleton (function)
macro_rules! Depcrate_utilmatch_singleton {
() => {
// Module: crate::util
// Provides: {"match_singleton"}
// Dependencies: {}
# [doc = " Returns `Some(x)` iff the iterable is singleton and otherwise None."] pub fn match_singleton < T > (it : impl IntoIterator < Item = T >) -> Option < T > { let mut it = it . into_iter () ; it . next () . filter (| _ | it . next () . is_none ()) }
};
}
