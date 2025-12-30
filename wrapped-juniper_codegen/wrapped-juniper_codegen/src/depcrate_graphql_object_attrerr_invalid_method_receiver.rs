// Generated macro for err_invalid_method_receiver (function)
macro_rules! Depcrate_graphql_object_attrerr_invalid_method_receiver {
() => {
// Module: crate::graphql_object::attr
// Provides: {"err_invalid_method_receiver"}
// Dependencies: {}
# [doc = " Emits \"invalid method receiver\" [`syn::Error`] pointing to the given `span`."] # [must_use] fn err_invalid_method_receiver < T , S : Spanned > (span : & S) -> Option < T > { ERR . emit_custom (span . span () , "method should have a shared reference receiver `&self`, or no receiver at all" ,) ; None }
};
}
