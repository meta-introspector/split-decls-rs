// Generated macro for trie_unwrap (macro)
macro_rules! Depcrate_char16trie_trietrie_unwrap {
() => {
// Module: crate::char16trie::trie
// Provides: {"trie_unwrap"}
// Dependencies: {}
# [doc = " A macro that takes an `Option` argument and either unwraps it if it has a value or"] # [doc = " causes the function to return `TrieResult::NoMatch` if there is no value."] # [doc = " This could perhaps be done with `std::ops::Try` once stabilized."] macro_rules ! trie_unwrap { ($ option : expr) => { match $ option { Some (x) => x , None => { debug_assert ! (false) ; return TrieResult :: NoMatch ; } } } ; }
};
}
