macro_rules! trie_value_indicates_special_non_starter_decomposition {
    () => {
        # [doc = " Checks if the trie signifies a special non-starter decomposition."] # [doc = ""] # [doc = " See trie-value-format.md"] fn trie_value_indicates_special_non_starter_decomposition (trie_value : u32) -> bool { (trie_value & 0x3FFFFF00) == 0xD900 }
    };
}

trie_value_indicates_special_non_starter_decomposition!();