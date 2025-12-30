// Generated macro for sort_slice_by_ccc (function)
macro_rules! Depcratesort_slice_by_ccc {
() => {
// Module: crate
// Provides: {"sort_slice_by_ccc"}
// Dependencies: {}
# [inline (always)] fn sort_slice_by_ccc (slice : & mut [CharacterAndClass] , trie : & Trie) { if slice . len () < 2 { return ; } slice . iter_mut () . for_each (| cc | cc . set_ccc_from_trie_if_not_already_set (trie)) ; slice . sort_by_key (| cc | cc . ccc ()) ; }
};
}
