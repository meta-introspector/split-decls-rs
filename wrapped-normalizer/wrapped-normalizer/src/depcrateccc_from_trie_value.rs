// Generated macro for ccc_from_trie_value (function)
macro_rules! Depcrateccc_from_trie_value {
() => {
// Module: crate
// Provides: {"ccc_from_trie_value"}
// Dependencies: {}
# [doc = " Extracts a canonical combining class (possibly zero) from a trie value."] # [doc = ""] # [doc = " See trie-value-format.md"] fn ccc_from_trie_value (trie_value : u32) -> CanonicalCombiningClass { if trie_value_has_ccc (trie_value) { CanonicalCombiningClass :: from_icu4c_value (trie_value as u8) } else { CCC_NOT_REORDERED } }
};
}
