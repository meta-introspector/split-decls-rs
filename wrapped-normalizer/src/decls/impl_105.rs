macro_rules! deps {
    () => {
        CharacterAndTrieValue!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl CharacterAndTrieValue { # [inline (always)] pub fn new (c : char , trie_value : u32) -> Self { CharacterAndTrieValue { character : c , trie_val : trie_value , } } # [inline (always)] pub fn starter_and_decomposes_to_self (& self) -> bool { starter_and_decomposes_to_self_impl (self . trie_val) } # [doc = " See trie-value-format.md"] # [inline (always)] # [cfg (feature = "utf8_iter")] pub fn starter_and_decomposes_to_self_except_replacement (& self) -> bool { (self . trie_val & ! BACKWARD_COMBINING_MARKER) == 0 } # [doc = " See trie-value-format.md"] # [inline (always)] pub fn can_combine_backwards (& self) -> bool { (self . trie_val & BACKWARD_COMBINING_MARKER) != 0 } # [doc = " See trie-value-format.md"] # [inline (always)] pub fn potential_passthrough (& self) -> bool { (self . trie_val & NON_ROUND_TRIP_MARKER) == 0 } # [doc = " See trie-value-format.md"] # [inline (always)] pub fn potential_passthrough_and_cannot_combine_backwards (& self) -> bool { potential_passthrough_and_cannot_combine_backwards_impl (self . trie_val) } }
    };
}

impl_105!()