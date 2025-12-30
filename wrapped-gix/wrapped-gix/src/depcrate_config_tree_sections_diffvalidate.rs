// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_diffvalidate {
() => {
// Module: crate::config::tree::sections::diff
// Provides: {"validate"}
// Dependencies: {}
pub (super) mod validate { use crate :: { bstr :: BStr , config :: tree :: { keys , Diff } , } ; pub struct Ignore ; impl keys :: Validate for Ignore { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { gix_submodule :: config :: Ignore :: try_from (value) . map_err (| () | format ! ("Value '{value}' is not a valid submodule 'ignore' value")) ? ; Ok (()) } } pub struct Algorithm ; impl keys :: Validate for Algorithm { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Diff :: ALGORITHM . try_into_algorithm (value . into ()) ? ; Ok (()) } } pub struct Renames ; impl keys :: Validate for Renames { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { let boolean = gix_config :: Boolean :: try_from (value) . map (| b | b . 0) ; Diff :: RENAMES . try_into_renames (boolean) ? ; Ok (()) } } pub struct Binary ; impl keys :: Validate for Binary { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Diff :: DRIVER_BINARY . try_into_binary (Some (value . into ())) ? ; Ok (()) } } }
};
}
