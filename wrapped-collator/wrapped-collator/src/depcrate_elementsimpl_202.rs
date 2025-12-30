// Generated macro for impl_202 (impl)
macro_rules! Depcrate_elementsimpl_202 {
() => {
// Module: crate::elements
// Provides: {"impl_202"}
// Dependencies: {}
impl CharacterAndClassAndTrieValue { pub fn new_with_non_decomposing_starter (c : char) -> Self { CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass :: new (c , CanonicalCombiningClass :: NotReordered) , trie_val : 0 , } } pub fn new_with_non_zero_ccc (c : char , ccc : CanonicalCombiningClass) -> Self { CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass :: new (c , ccc) , trie_val : 0xD800 | u32 :: from (ccc . to_icu4c_value ()) , } } pub fn new_with_non_special_decomposition_trie_val (c : char , trie_val : u32) -> Self { debug_assert ! (! trie_value_indicates_special_non_starter_decomposition (trie_val)) ; CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass :: new_with_trie_value (c , trie_val) , trie_val , } } pub fn new_with_trie_val (c : char , trie_val : u32) -> Self { if ! trie_value_indicates_special_non_starter_decomposition (trie_val) { CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass :: new_with_trie_value (c , trie_val) , trie_val , } } else { CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass :: new (c , CanonicalCombiningClass :: from_icu4c_value (0xFF)) , trie_val , } } } pub fn decomposition_starts_with_non_starter (& self) -> bool { decomposition_starts_with_non_starter (self . trie_val) } pub fn character (& self) -> char { self . c_and_c . character () } fn ccc (& self) -> CanonicalCombiningClass { let ret = self . c_and_c . ccc () ; debug_assert_ne ! (ret , CanonicalCombiningClass :: from_icu4c_value (0xFF)) ; ret } }
};
}
