// Generated macro for Cache (struct)
macro_rules! Depcrate_dfaCache {
() => {
// Module: crate::dfa
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A reusable cache of DFA states."] # [doc = ""] # [doc = " This cache is reused between multiple invocations of the same regex"] # [doc = " program. (It is not shared simultaneously between threads. If there is"] # [doc = " contention, then new caches are created.)"] # [derive (Clone , Debug)] pub struct Cache { # [doc = " Group persistent DFA related cache state together. The sparse sets"] # [doc = " listed below are used as scratch space while computing uncached states."] inner : CacheInner , # [doc = " qcur and qnext are ordered sets with constant time"] # [doc = " addition/membership/clearing-whole-set and linear time iteration. They"] # [doc = " are used to manage the sets of NFA states in DFA states when computing"] # [doc = " cached DFA states. In particular, the order of the NFA states matters"] # [doc = " for leftmost-first style matching. Namely, when computing a cached"] # [doc = " state, the set of NFA states stops growing as soon as the first Match"] # [doc = " instruction is observed."] qcur : SparseSet , qnext : SparseSet , }
};
}
