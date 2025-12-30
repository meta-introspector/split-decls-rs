// Generated macro for impl_222 (impl)
macro_rules! Depcrate_codepointtrie_tomlimpl_222 {
() => {
// Module: crate::codepointtrie::toml
// Provides: {"impl_222"}
// Dependencies: {}
impl < T : TrieValue > TryFrom < & CodePointTrieToml > for CodePointTrie < 'static , T > { type Error = Error ; fn try_from (cpt_data : & CodePointTrieToml) -> Result < CodePointTrie < 'static , T > , Self :: Error > { use CodePointDataSlice :: * ; let header = CodePointTrieHeader :: try_from (cpt_data) ? ; let index : ZeroVec < u16 > = ZeroVec :: alloc_from_slice (& cpt_data . index) ; let data : Result < ZeroVec < 'static , T > , T :: TryFromU32Error > = match cpt_data . data_slice () ? { U8 (s) => s . iter () . map (| i | T :: try_from_u32 (* i as u32)) . collect () , U16 (s) => s . iter () . map (| i | T :: try_from_u32 (* i as u32)) . collect () , U32 (s) => s . iter () . map (| i | T :: try_from_u32 (* i)) . collect () , } ; let data = data . map_err (| _ | Error :: FromDeserialized { reason : "Could not parse data array to typed array" , }) ? ; CodePointTrie :: < T > :: try_new (header , index , data) } }
};
}
