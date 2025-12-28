macro_rules! trie_value_has_ccc {
    () => {
        # [doc = " Checks if a trie value carries a (non-zero) canonical"] # [doc = " combining class."] # [doc = ""] # [doc = " See trie-value-format.md"] fn trie_value_has_ccc (trie_value : u32) -> bool { (trie_value & 0x3FFFFE00) == 0xD800 }
    };
}

trie_value_has_ccc!();