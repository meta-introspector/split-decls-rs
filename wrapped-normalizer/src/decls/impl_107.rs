macro_rules! deps {
    () => {
        CharacterAndClass!();
        CanonicalCombiningClass!();
        Trie!();
        CharacterAndTrieValue!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl CharacterAndClass { pub fn new (c : char , ccc : CanonicalCombiningClass) -> Self { CharacterAndClass (u32 :: from (c) | (u32 :: from (ccc . to_icu4c_value ()) << 24)) } pub fn new_with_placeholder (c : char) -> Self { CharacterAndClass (u32 :: from (c) | ((0xFF) << 24)) } pub fn new_with_trie_value (c_tv : CharacterAndTrieValue) -> Self { Self :: new (c_tv . character , ccc_from_trie_value (c_tv . trie_val)) } pub fn new_starter (c : char) -> Self { CharacterAndClass (u32 :: from (c)) } # [doc = " This method must exist for Pernosco to apply its special rendering."] # [doc = " Also, this must not be dead code!"] pub fn character (& self) -> char { unsafe { char :: from_u32_unchecked (self . 0 & 0xFFFFFF) } } # [doc = " This method must exist for Pernosco to apply its special rendering."] pub fn ccc (& self) -> CanonicalCombiningClass { CanonicalCombiningClass :: from_icu4c_value ((self . 0 >> 24) as u8) } pub fn character_and_ccc (& self) -> (char , CanonicalCombiningClass) { (self . character () , self . ccc ()) } pub fn set_ccc_from_trie_if_not_already_set (& mut self , trie : & Trie) { if self . 0 >> 24 != 0xFF { return ; } let scalar = self . 0 & 0xFFFFFF ; self . 0 = ((ccc_from_trie_value (trie . get32_u32 (scalar)) . to_icu4c_value () as u32) << 24) | scalar ; } }
    };
}

impl_107!()