// Generated macro for quote_optional_label (function)
macro_rules! Depcrate_labelquote_optional_label {
() => {
// Module: crate::label
// Provides: {"quote_optional_label"}
// Dependencies: {}
pub (crate) fn quote_optional_label (label : Option < Label >) -> proc_macro2 :: TokenStream { match label { Some (label) => { let label = quote_label (label) ; quote ! (Some (# label)) } None => quote ! (None) , } }
};
}
