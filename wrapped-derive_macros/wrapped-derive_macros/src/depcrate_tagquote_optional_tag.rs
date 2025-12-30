// Generated macro for quote_optional_tag (function)
macro_rules! Depcrate_tagquote_optional_tag {
() => {
// Module: crate::tag
// Provides: {"quote_optional_tag"}
// Dependencies: {}
pub (crate) fn quote_optional_tag (tag : Option < & Path >) -> proc_macro2 :: TokenStream { match tag { Some (tag) => quote ! (Some (&# tag)) , None => quote ! (None) , } }
};
}
