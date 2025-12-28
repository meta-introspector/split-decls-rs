macro_rules! Trie {
    () => {
        # [cfg (icu4x_unstable_fast_trie_only)] type Trie < 'trie > = FastCodePointTrie < 'trie , u32 > ;
    };
}

Trie!();