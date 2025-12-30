// Generated macro for err_unnamed_field (function)
macro_rules! Depcrate_graphql_interface_attrerr_unnamed_field {
() => {
// Module: crate::graphql_interface::attr
// Provides: {"err_unnamed_field"}
// Dependencies: {}
# [doc = " Emits \"expected named struct field\" [`syn::Error`] pointing to the given"] # [doc = " `span`."] pub (crate) fn err_unnamed_field < T , S : Spanned > (span : & S) -> Option < T > { ERR . emit_custom (span . span () , "expected named struct field") ; None }
};
}
