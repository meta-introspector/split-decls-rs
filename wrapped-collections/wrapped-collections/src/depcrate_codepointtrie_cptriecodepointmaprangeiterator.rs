// Generated macro for CodePointMapRangeIterator (struct)
macro_rules! Depcrate_codepointtrie_cptrieCodePointMapRangeIterator {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"CodePointMapRangeIterator"}
// Dependencies: {}
# [doc = " A custom [`Iterator`] type specifically for a code point trie that returns"] # [doc = " [`CodePointMapRange`]s."] pub struct CodePointMapRangeIterator < 'a , T : TrieValue > { cpt : & 'a CodePointTrie < 'a , T > , cpm_range : Option < CodePointMapRange < T > > , }
};
}
