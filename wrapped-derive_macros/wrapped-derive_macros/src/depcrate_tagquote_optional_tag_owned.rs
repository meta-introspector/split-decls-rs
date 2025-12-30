// Generated macro for quote_optional_tag_owned (function)
macro_rules! Depcrate_tagquote_optional_tag_owned {
() => {
// Module: crate::tag
// Provides: {"quote_optional_tag_owned"}
// Dependencies: {}
pub (crate) fn quote_optional_tag_owned (tag : Option < & Path >) -> proc_macro2 :: TokenStream { match tag { Some (tag) => quote ! (Some (# tag)) , None => quote ! (None) , } }
};
}
