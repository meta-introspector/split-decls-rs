// Generated macro for Char16Trie (struct)
macro_rules! Depcrate_char16trie_trieChar16Trie {
() => {
// Module: crate::char16trie::trie
// Provides: {"Char16Trie"}
// Dependencies: {}
# [doc = " This struct represents a de-serialized `Char16Trie` that was exported from"] # [doc = " ICU binary data."] # [doc = ""] # [doc = " Light-weight, non-const reader class for a `CharsTrie`. Traverses a"] # [doc = " char-serialized data structure with minimal state, for mapping 16-bit-unit"] # [doc = " sequences to non-negative integer values."] # [doc = ""] # [doc = " For more information:"] # [doc = " - [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)"] # [doc = " - [ICU4J CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API."] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [cfg_attr (feature = "databake" , derive (databake :: Bake))] # [cfg_attr (feature = "databake" , databake (path = icu_collections :: char16trie))] # [derive (Clone , Debug , PartialEq , Eq , ZeroFrom)] pub struct Char16Trie < 'data > { # [doc = " An array of u16 containing the trie data."] # [cfg_attr (feature = "serde" , serde (borrow))] # [doc (hidden)] pub data : ZeroVec < 'data , u16 > , }
};
}
