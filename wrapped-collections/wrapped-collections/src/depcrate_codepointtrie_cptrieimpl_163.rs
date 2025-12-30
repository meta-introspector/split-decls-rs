// Generated macro for impl_163 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_163 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TryFrom < & 'trie CodePointTrie < 'trie , T > > for & 'trie FastCodePointTrie < 'trie , T > { type Error = TypedCodePointTrieError ; fn try_from (reference : & 'trie CodePointTrie < 'trie , T > ,) -> Result < & 'trie FastCodePointTrie < 'trie , T > , TypedCodePointTrieError > { match reference . as_typed_ref () { Typed :: Fast (trie) => Ok (trie) , Typed :: Small (_) => Err (TypedCodePointTrieError) , } } }
};
}
