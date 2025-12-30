// Generated macro for QueuedSet (struct)
macro_rules! Depcrate_nfa_noncontiguousQueuedSet {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"QueuedSet"}
// Dependencies: {}
# [doc = " A set of state identifiers used to avoid revisiting the same state multiple"] # [doc = " times when filling in failure transitions."] # [doc = ""] # [doc = " This set has an \"inert\" and an \"active\" mode. When inert, the set never"] # [doc = " stores anything and always returns `false` for every member test. This is"] # [doc = " useful to avoid the performance and memory overhead of maintaining this"] # [doc = " set when it is not needed."] # [derive (Debug)] struct QueuedSet { set : Option < BTreeSet < StateID > > , }
};
}
