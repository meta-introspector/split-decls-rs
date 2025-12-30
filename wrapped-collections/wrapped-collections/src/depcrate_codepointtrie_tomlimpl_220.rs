// Generated macro for impl_220 (impl)
macro_rules! Depcrate_codepointtrie_tomlimpl_220 {
() => {
// Module: crate::codepointtrie::toml
// Provides: {"impl_220"}
// Dependencies: {}
impl CodePointTrieToml { # [doc = " Gets the `index` slice."] pub fn index_slice (& self) -> & [u16] { self . index . as_slice () } # [doc = " Gets the `data` slice."] pub fn data_slice (& self) -> Result < CodePointDataSlice < '_ > , Error > { if let Some (data_8) = & self . data_8 { Ok (CodePointDataSlice :: U8 (data_8 . as_slice ())) } else if let Some (data_16) = & self . data_16 { Ok (CodePointDataSlice :: U16 (data_16 . as_slice ())) } else if let Some (data_32) = & self . data_32 { Ok (CodePointDataSlice :: U32 (data_32 . as_slice ())) } else { Err (Error :: FromDeserialized { reason : "Did not find data array for CodePointTrie in TOML" , }) } } }
};
}
