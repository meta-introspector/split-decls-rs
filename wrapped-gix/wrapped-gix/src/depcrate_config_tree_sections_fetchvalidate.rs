// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_fetchvalidate {
() => {
// Module: crate::config::tree::sections::fetch
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct NegotiationAlgorithm ; impl keys :: Validate for NegotiationAlgorithm { # [cfg_attr (not (feature = "credentials") , allow (unused_variables))] fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { # [cfg (feature = "credentials")] crate :: config :: tree :: Fetch :: NEGOTIATION_ALGORITHM . try_into_negotiation_algorithm (value . into ()) ? ; Ok (()) } } # [cfg (feature = "attributes")] pub struct RecurseSubmodules ; # [cfg (feature = "attributes")] impl keys :: Validate for RecurseSubmodules { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { { let boolean = gix_config :: Boolean :: try_from (value) . map (| b | b . 0) ; crate :: config :: tree :: Fetch :: RECURSE_SUBMODULES . try_into_recurse_submodules (boolean) ? ; } Ok (()) } } }
};
}
