// Generated macro for impl_164 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_164 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TryFrom < CodePointTrie < 'trie , T > > for FastCodePointTrie < 'trie , T > { type Error = TypedCodePointTrieError ; fn try_from (value : CodePointTrie < 'trie , T > ,) -> Result < FastCodePointTrie < 'trie , T > , TypedCodePointTrieError > { match value . to_typed () { Typed :: Fast (trie) => Ok (trie) , Typed :: Small (_) => Err (TypedCodePointTrieError) , } } }
};
}
