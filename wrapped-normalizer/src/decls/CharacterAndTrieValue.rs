macro_rules! CharacterAndTrieValue {
    () => {
        # [doc = " Struct for holding together a character and the value"] # [doc = " looked up for it from the NFD trie in a more explicit"] # [doc = " way than an anonymous pair."] # [doc = " Also holds a flag about the supplementary-trie provenance."] # [derive (Debug , PartialEq , Eq)] struct CharacterAndTrieValue { character : char , # [doc = " See trie-value-format.md"] trie_val : u32 , }
    };
}

CharacterAndTrieValue!();