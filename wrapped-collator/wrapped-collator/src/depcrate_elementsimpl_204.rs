// Generated macro for impl_204 (impl)
macro_rules! Depcrate_elementsimpl_204 {
() => {
// Module: crate::elements
// Provides: {"impl_204"}
// Dependencies: {}
impl CharacterAndClass { pub fn new (c : char , ccc : CanonicalCombiningClass) -> Self { CharacterAndClass (u32 :: from (c) | (u32 :: from (ccc . to_icu4c_value ()) << 24)) } pub fn new_with_placeholder (c : char) -> Self { CharacterAndClass (u32 :: from (c) | ((0xFF) << 24)) } pub fn new_with_trie_value (c : char , trie_value : u32) -> Self { Self :: new (c , ccc_from_trie_value (trie_value)) } pub fn character (& self) -> char { unsafe { char :: from_u32_unchecked (self . 0 & 0xFF_FFFF) } } pub fn ccc (& self) -> CanonicalCombiningClass { CanonicalCombiningClass :: from_icu4c_value ((self . 0 >> 24) as u8) } pub fn character_and_ccc (& self) -> (char , CanonicalCombiningClass) { (self . character () , self . ccc ()) } pub fn set_ccc_from_trie_if_not_already_set (& mut self , trie : & CodePointTrie < u32 >) { if self . 0 >> 24 != 0xFF { return ; } let scalar = self . 0 & 0xFF_FFFF ; self . 0 = ((ccc_from_trie_value (trie . get32_u32 (scalar)) . to_icu4c_value () as u32) << 24) | scalar ; } }
};
}
