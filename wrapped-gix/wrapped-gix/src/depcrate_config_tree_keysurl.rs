// Generated macro for url (module)
macro_rules! Depcrate_config_tree_keysurl {
() => {
// Module: crate::config::tree::keys
// Provides: {"url"}
// Dependencies: {}
mod url { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: { keys :: { validate , Url } , Section , } , } ; impl Url { # [doc = " Create a new instance."] pub const fn new_url (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Url) } # [doc = " Try to parse `value` as URL."] pub fn try_into_url (& 'static self , value : Cow < '_ , BStr >) -> Result < gix_url :: Url , config :: url :: Error > { gix_url :: parse (value . as_ref ()) . map_err (| err | config :: url :: Error :: from_value (self , value . into_owned ()) . with_source (err)) } } }
};
}
