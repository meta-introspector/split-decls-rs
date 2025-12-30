// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_statusvalidate {
() => {
// Module: crate::config::tree::sections::status
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct ShowUntrackedFiles ; impl keys :: Validate for ShowUntrackedFiles { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { super :: Status :: SHOW_UNTRACKED_FILES . try_into_show_untracked_files (value . into ()) ? ; Ok (()) } } }
};
}
