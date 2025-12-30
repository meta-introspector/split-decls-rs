// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_gitoxidevalidate {
() => {
// Module: crate::config::tree::sections::gitoxide
// Provides: {"validate"}
// Dependencies: {}
pub mod validate { use std :: error :: Error ; use crate :: { bstr :: BStr , config :: tree :: keys :: Validate } ; pub struct ProtocolFromUser ; impl Validate for ProtocolFromUser { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { if value != "1" { return Err ("GIT_PROTOCOL_FROM_USER is either unset or as the value '1'" . into ()) ; } Ok (()) } } pub struct RefsNamespace ; impl Validate for RefsNamespace { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { super :: Core :: REFS_NAMESPACE . try_into_refs_namespace (value . into ()) ? ; Ok (()) } } }
};
}
