// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_packvalidate {
() => {
// Module: crate::config::tree::sections::pack
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct IndexVersion ; impl keys :: Validate for IndexVersion { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { super :: Pack :: INDEX_VERSION . try_into_index_version (gix_config :: Integer :: try_from (value) . and_then (| int | { int . to_decimal () . ok_or_else (| | gix_config :: value :: Error :: new ("integer out of range" , value)) } ,)) ? ; Ok (()) } } }
};
}
