// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_remotevalidate {
() => {
// Module: crate::config::tree::sections::remote
// Provides: {"validate"}
// Dependencies: {}
pub mod validate { use std :: { borrow :: Cow , error :: Error } ; use crate :: { bstr :: BStr , config :: tree :: keys :: Validate } ; pub struct TagOpt ; impl Validate for TagOpt { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { super :: Remote :: TAG_OPT . try_into_tag_opt (Cow :: Borrowed (value)) ? ; Ok (()) } } }
};
}
