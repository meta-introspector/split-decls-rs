// Generated macro for trie_value_indicates_special_non_starter_decomposition (function)
macro_rules! Depcratetrie_value_indicates_special_non_starter_decomposition {
() => {
// Module: crate
// Provides: {"trie_value_indicates_special_non_starter_decomposition"}
// Dependencies: {}
# [doc = " Checks if the trie signifies a special non-starter decomposition."] # [doc = ""] # [doc = " See trie-value-format.md"] fn trie_value_indicates_special_non_starter_decomposition (trie_value : u32) -> bool { (trie_value & 0x3FFFFF00) == 0xD900 }
};
}
