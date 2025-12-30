// Generated macro for impl_168 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_168 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TryFrom < & 'trie CodePointTrie < 'trie , T > > for & 'trie SmallCodePointTrie < 'trie , T > { type Error = TypedCodePointTrieError ; fn try_from (reference : & 'trie CodePointTrie < 'trie , T > ,) -> Result < & 'trie SmallCodePointTrie < 'trie , T > , TypedCodePointTrieError > { match reference . as_typed_ref () { Typed :: Fast (_) => Err (TypedCodePointTrieError) , Typed :: Small (trie) => Ok (trie) , } } }
};
}
