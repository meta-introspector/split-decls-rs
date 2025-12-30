// Generated macro for impl_primitive_trie_value (macro)
macro_rules! Depcrate_codepointtrie_cptrieimpl_primitive_trie_value {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_primitive_trie_value"}
// Dependencies: {}
macro_rules ! impl_primitive_trie_value { ($ primitive : ty , $ error : ty) => { impl TrieValue for $ primitive { type TryFromU32Error = $ error ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { Self :: try_from (i) } fn to_u32 (self) -> u32 { self as u32 } } } ; }
};
}
