// Generated macro for err_unnamed_field (function)
macro_rules! Depcrate_graphql_input_object_deriveerr_unnamed_field {
() => {
// Module: crate::graphql_input_object::derive
// Provides: {"err_unnamed_field"}
// Dependencies: {}
# [doc = " Emits \"expected named struct field\" [`syn::Error`] pointing to the provided `span`."] pub (crate) fn err_unnamed_field < T , S : Spanned > (span : & S) -> Option < T > { ERR . emit_custom (span . span () , "expected named struct field") ; None }
};
}
