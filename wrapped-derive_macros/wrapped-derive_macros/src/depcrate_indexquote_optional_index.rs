// Generated macro for quote_optional_index (function)
macro_rules! Depcrate_indexquote_optional_index {
() => {
// Module: crate::index
// Provides: {"quote_optional_index"}
// Dependencies: {}
pub (crate) fn quote_optional_index (index : Option < Index >) -> proc_macro2 :: TokenStream { match index { Some (index) => { let index = quote_index (index) ; quote ! (Some (# index)) } None => quote ! (None) , } }
};
}
