// Generated macro for impl_58 (impl)
macro_rules! Depcrate_match_groupimpl_58 {
() => {
// Module: crate::match_group
// Provides: {"impl_58"}
// Dependencies: {}
# [doc = " Initialization"] impl < 'a > MatchGroup < 'a > { # [doc = " Take all the fetch ref specs from `specs` get a match group ready."] pub fn from_fetch_specs (specs : impl IntoIterator < Item = RefSpecRef < 'a > >) -> Self { MatchGroup { specs : specs . into_iter () . filter (| s | s . op == Operation :: Fetch) . collect () , } } # [doc = " Take all the push ref specs from `specs` get a match group ready."] pub fn from_push_specs (specs : impl IntoIterator < Item = RefSpecRef < 'a > >) -> Self { MatchGroup { specs : specs . into_iter () . filter (| s | s . op == Operation :: Push) . collect () , } } }
};
}
