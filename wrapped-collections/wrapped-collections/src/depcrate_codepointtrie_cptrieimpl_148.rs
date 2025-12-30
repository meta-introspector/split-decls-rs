// Generated macro for impl_148 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_148 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_148"}
// Dependencies: {}
impl TryFrom < u8 > for TrieType { type Error = crate :: codepointtrie :: error :: Error ; fn try_from (trie_type_int : u8) -> Result < TrieType , crate :: codepointtrie :: error :: Error > { match trie_type_int { 0 => Ok (TrieType :: Fast) , 1 => Ok (TrieType :: Small) , _ => Err (crate :: codepointtrie :: error :: Error :: FromDeserialized { reason : "Cannot parse value for trie_type" , }) , } } }
};
}
