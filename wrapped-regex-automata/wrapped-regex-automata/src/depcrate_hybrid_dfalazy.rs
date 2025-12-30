// Generated macro for Lazy (struct)
macro_rules! Depcrate_hybrid_dfaLazy {
() => {
// Module: crate::hybrid::dfa
// Provides: {"Lazy"}
// Dependencies: {}
# [doc = " A type that groups methods that require the base NFA/DFA and writable"] # [doc = " access to the cache."] # [derive (Debug)] struct Lazy < 'i , 'c > { dfa : & 'i DFA , cache : & 'c mut Cache , }
};
}
