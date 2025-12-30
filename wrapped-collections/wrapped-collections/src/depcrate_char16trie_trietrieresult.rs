// Generated macro for TrieResult (enum)
macro_rules! Depcrate_char16trie_trieTrieResult {
() => {
// Module: crate::char16trie::trie
// Provides: {"TrieResult"}
// Dependencies: {}
# [doc = " An enum representing the return value from a lookup in [`Char16Trie`]."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum TrieResult { # [doc = " The input unit(s) did not continue a matching string."] # [doc = " Once `next()` returns `TrieResult::NoMatch`, all further calls to `next()`"] # [doc = " will also return `TrieResult::NoMatch`."] NoMatch , # [doc = " The input unit(s) matched a string but there is no value for the string"] # [doc = " so far.  (It is a prefix of a longer string.)"] NoValue , # [doc = " The input unit(s) continued a matching string and there is a value for"] # [doc = " the string so far. No further input byte/unit can continue a matching"] # [doc = " string."] FinalValue (i32) , # [doc = " The input unit(s) continued a matching string and there is a value for"] # [doc = " the string so far.  Another input byte/unit can continue a matching"] # [doc = " string."] Intermediate (i32) , }
};
}
