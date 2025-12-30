// Generated macro for impl_161 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_161 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'trie , T : TrieValue > TypedCodePointTrie < 'trie , T > for FastCodePointTrie < 'trie , T > { const TRIE_TYPE : TrieType = TrieType :: Fast ; # [doc = " Returns a reference to the wrapped `CodePointTrie`."] # [inline (always)] fn as_untyped_ref (& self) -> & CodePointTrie < 'trie , T > { & self . inner } # [doc = " Extracts the wrapped `CodePointTrie`."] # [inline (always)] fn to_untyped (self) -> CodePointTrie < 'trie , T > { self . inner } # [doc = " Lookup trie value by Basic Multilingual Plane Code Point without branching on trie type."] # [inline (always)] fn get16 (& self , bmp : u16) -> T { debug_assert ! (u32 :: from (u16 :: MAX) <= FAST_TYPE_FAST_INDEXING_MAX) ; debug_assert_eq ! (Self :: TRIE_TYPE , TrieType :: Fast) ; debug_assert_eq ! (self . as_untyped_ref () . header . trie_type , TrieType :: Fast) ; let code_point = u32 :: from (bmp) ; unsafe { self . as_untyped_ref () . get32_assuming_fast_index (code_point) } } }
};
}
