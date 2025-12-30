// Generated macro for with_ref (function)
macro_rules! Depcrate_item_typewith_ref {
() => {
// Module: crate::item_type
// Provides: {"with_ref"}
// Dependencies: {}
fn with_ref (source : & impl ToTokens , is_ref : bool) -> TokenStream { if is_ref { quote ! (&# source) } else { quote ! (# source) } }
};
}
