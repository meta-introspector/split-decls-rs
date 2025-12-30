// Generated macro for Cache (struct)
macro_rules! Depcrate_pikevmCache {
() => {
// Module: crate::pikevm
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A cached allocation that can be reused on each execution."] # [derive (Clone , Debug)] pub struct Cache { # [doc = " A pair of ordered sets for tracking NFA states."] clist : Threads , nlist : Threads , # [doc = " An explicit stack used for following epsilon transitions."] stack : Vec < FollowEpsilon > , }
};
}
