// Generated macro for impl_230 (impl)
macro_rules! Depcrate_codepointtrie_serdeimpl_230 {
() => {
// Module: crate::codepointtrie::serde
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'de , 'trie , T : TrieValue + Deserialize < 'de > > Deserialize < 'de > for CodePointTrie < 'trie , T > where 'de : 'trie , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let de = CodePointTrieSerde :: deserialize (deserializer) ? ; let error_value = match CodePointTrie :: validate_fields (& de . header , & de . index , & de . data) { Ok (v) => v , Err (e) => { match e { super :: CodePointTrieError :: FromDeserialized { reason } => { debug_assert ! (false) ; return Err (D :: Error :: custom (reason)) ; } super :: CodePointTrieError :: EmptyDataVector => { return Err (D :: Error :: custom ("CodePointTrie must be constructed from data vector with at least one element")) ; } super :: CodePointTrieError :: IndexTooShortForFastAccess => { return Err (D :: Error :: custom ("CodePointTrie must be constructed from index vector long enough to accommodate fast-path access")) ; } super :: CodePointTrieError :: DataTooShortForFastAccess => { return Err (D :: Error :: custom ("CodePointTrie must be constructed from data vector long enough to accommodate fast-path access")) ; } } } } ; Ok (CodePointTrie { header : de . header , index : de . index , data : de . data , error_value , }) } }
};
}
