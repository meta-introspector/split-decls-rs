macro_rules! deps {
    () => {
        CharacterAndClass!();
        Trie!();
    };
}

macro_rules! sort_slice_by_ccc {
    () => {
        deps!();
        # [inline (always)] fn sort_slice_by_ccc (slice : & mut [CharacterAndClass] , trie : & Trie) { if slice . len () < 2 { return ; } slice . iter_mut () . for_each (| cc | cc . set_ccc_from_trie_if_not_already_set (trie)) ; slice . sort_by_key (| cc | cc . ccc ()) ; }
    };
}

sort_slice_by_ccc!();