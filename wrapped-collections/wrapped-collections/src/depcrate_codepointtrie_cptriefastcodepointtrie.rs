// Generated macro for FastCodePointTrie (struct)
macro_rules! Depcrate_codepointtrie_cptrieFastCodePointTrie {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"FastCodePointTrie"}
// Dependencies: {}
# [doc = " Type-safe wrapper for a fast trie guaranteeing"] # [doc = " the the getters don't branch on the trie type"] # [doc = " and for guarenteeing that `get16` is branchless"] # [doc = " in release builds."] # [derive (Debug)] # [repr (transparent)] pub struct FastCodePointTrie < 'trie , T : TrieValue > { inner : CodePointTrie < 'trie , T > , }
};
}
