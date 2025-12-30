// Generated macro for index_version (module)
macro_rules! Depcrate_config_tree_sections_packindex_version {
() => {
// Module: crate::config::tree::sections::pack
// Provides: {"index_version"}
// Dependencies: {}
mod index_version { use crate :: { config , config :: tree :: sections :: pack :: IndexVersion } ; impl IndexVersion { # [doc = " Try to interpret an integer value as index version."] pub fn try_into_index_version (& 'static self , value : Result < i64 , gix_config :: value :: Error > ,) -> Result < gix_pack :: index :: Version , config :: key :: GenericError > { let value = value . map_err (| err | config :: key :: GenericError :: from (self) . with_source (err)) ? ; Ok (match value { 1 => gix_pack :: index :: Version :: V1 , 2 => gix_pack :: index :: Version :: V2 , _ => return Err (config :: key :: GenericError :: from (self)) , }) } } }
};
}
