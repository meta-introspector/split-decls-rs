// Generated macro for impl_221 (impl)
macro_rules! Depcrate_codepointtrie_tomlimpl_221 {
() => {
// Module: crate::codepointtrie::toml
// Provides: {"impl_221"}
// Dependencies: {}
impl TryFrom < & CodePointTrieToml > for CodePointTrieHeader { type Error = Error ; fn try_from (cpt_data : & CodePointTrieToml) -> Result < Self , Self :: Error > { let trie_type_enum : TrieType = TrieType :: try_from (cpt_data . trie_type_enum_val) ? ; Ok (CodePointTrieHeader { high_start : cpt_data . high_start , shifted12_high_start : cpt_data . shifted12_high_start , index3_null_offset : cpt_data . index3_null_offset , data_null_offset : cpt_data . data_null_offset , null_value : cpt_data . null_value , trie_type : trie_type_enum , }) } }
};
}
