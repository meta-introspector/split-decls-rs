// Generated macro for SearchProgress (struct)
macro_rules! Depcrate_hybrid_dfaSearchProgress {
() => {
// Module: crate::hybrid::dfa
// Provides: {"SearchProgress"}
// Dependencies: {}
# [doc = " Keeps track of the progress of the current search."] # [doc = ""] # [doc = " This is updated via the `Cache::search_{start,update,finish}` APIs to"] # [doc = " record how many bytes have been searched. This permits computing a"] # [doc = " heuristic that represents the efficiency of a cache, and thus helps inform"] # [doc = " whether the lazy DFA should give up or not."] # [derive (Clone , Debug)] struct SearchProgress { start : usize , at : usize , }
};
}
