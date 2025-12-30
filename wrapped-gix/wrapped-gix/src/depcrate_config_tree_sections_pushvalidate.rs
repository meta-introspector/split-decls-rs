// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_pushvalidate {
() => {
// Module: crate::config::tree::sections::push
// Provides: {"validate"}
// Dependencies: {}
mod validate { pub struct Default ; use std :: { borrow :: Cow , error :: Error } ; use crate :: { bstr :: BStr , config :: tree :: keys :: Validate } ; impl Validate for Default { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { super :: Push :: DEFAULT . try_into_default (Cow :: Borrowed (value)) ? ; Ok (()) } } }
};
}
