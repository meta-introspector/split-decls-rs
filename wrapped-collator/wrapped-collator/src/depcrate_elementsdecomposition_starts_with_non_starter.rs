// Generated macro for decomposition_starts_with_non_starter (function)
macro_rules! Depcrate_elementsdecomposition_starts_with_non_starter {
() => {
// Module: crate::elements
// Provides: {"decomposition_starts_with_non_starter"}
// Dependencies: {}
# [doc = " Checks if a trie value signifies a character whose decomposition"] # [doc = " starts with a non-starter."] # [doc = ""] # [doc = " See components/normalizer/trie-value-format.md"] fn decomposition_starts_with_non_starter (trie_value : u32) -> bool { trie_value_has_ccc (trie_value) }
};
}
