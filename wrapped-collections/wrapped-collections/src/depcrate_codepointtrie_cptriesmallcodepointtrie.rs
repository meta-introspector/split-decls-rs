// Generated macro for SmallCodePointTrie (struct)
macro_rules! Depcrate_codepointtrie_cptrieSmallCodePointTrie {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"SmallCodePointTrie"}
// Dependencies: {}
# [doc = " Type-safe wrapper for a small trie guaranteeing"] # [doc = " the the getters don't branch on the trie type."] # [derive (Debug)] # [repr (transparent)] pub struct SmallCodePointTrie < 'trie , T : TrieValue > { inner : CodePointTrie < 'trie , T > , }
};
}
