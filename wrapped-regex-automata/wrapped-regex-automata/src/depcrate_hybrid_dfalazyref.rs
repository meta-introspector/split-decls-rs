// Generated macro for LazyRef (struct)
macro_rules! Depcrate_hybrid_dfaLazyRef {
() => {
// Module: crate::hybrid::dfa
// Provides: {"LazyRef"}
// Dependencies: {}
# [doc = " A type that groups methods that require the base NFA/DFA and read-only"] # [doc = " access to the cache."] # [derive (Debug)] struct LazyRef < 'i , 'c > { dfa : & 'i DFA , cache : & 'c Cache , }
};
}
