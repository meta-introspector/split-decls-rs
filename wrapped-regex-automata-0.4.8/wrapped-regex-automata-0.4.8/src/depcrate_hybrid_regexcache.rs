// Generated macro for Cache (struct)
macro_rules! Depcrate_hybrid_regexCache {
() => {
// Module: crate::hybrid::regex
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A cache represents a partially computed forward and reverse DFA."] # [doc = ""] # [doc = " A cache is the key component that differentiates a classical DFA and a"] # [doc = " hybrid NFA/DFA (also called a \"lazy DFA\"). Where a classical DFA builds a"] # [doc = " complete transition table that can handle all possible inputs, a hybrid"] # [doc = " NFA/DFA starts with an empty transition table and builds only the parts"] # [doc = " required during search. The parts that are built are stored in a cache. For"] # [doc = " this reason, a cache is a required parameter for nearly every operation on"] # [doc = " a [`Regex`]."] # [doc = ""] # [doc = " Caches can be created from their corresponding `Regex` via"] # [doc = " [`Regex::create_cache`]. A cache can only be used with either the `Regex`"] # [doc = " that created it, or the `Regex` that was most recently used to reset it"] # [doc = " with [`Cache::reset`]. Using a cache with any other `Regex` may result in"] # [doc = " panics or incorrect results."] # [derive (Debug , Clone)] pub struct Cache { forward : dfa :: Cache , reverse : dfa :: Cache , }
};
}
