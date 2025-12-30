// Generated macro for impl_166 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_166 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TypedCodePointTrie < 'trie , T > for SmallCodePointTrie < 'trie , T > { const TRIE_TYPE : TrieType = TrieType :: Small ; # [doc = " Returns a reference to the wrapped `CodePointTrie`."] # [inline (always)] fn as_untyped_ref (& self) -> & CodePointTrie < 'trie , T > { & self . inner } # [doc = " Extracts the wrapped `CodePointTrie`."] # [inline (always)] fn to_untyped (self) -> CodePointTrie < 'trie , T > { self . inner } }
};
}
