// Generated macro for impl_169 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_169 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TryFrom < CodePointTrie < 'trie , T > > for SmallCodePointTrie < 'trie , T > { type Error = TypedCodePointTrieError ; fn try_from (value : CodePointTrie < 'trie , T > ,) -> Result < SmallCodePointTrie < 'trie , T > , TypedCodePointTrieError > { match value . to_typed () { Typed :: Fast (_) => Err (TypedCodePointTrieError) , Typed :: Small (trie) => Ok (trie) , } } }
};
}
