// Generated macro for err_default_impl_block (function)
macro_rules! Depcrate_graphql_interface_attrerr_default_impl_block {
() => {
// Module: crate::graphql_interface::attr
// Provides: {"err_default_impl_block"}
// Dependencies: {}
# [doc = " Emits \"trait method can't have default implementation\" [`syn::Error`]"] # [doc = " pointing to the given `span`."] fn err_default_impl_block < T , S : Spanned > (span : & S) -> Option < T > { ERR . emit_custom (span . span () , "trait method can't have default implementation" ,) ; None }
};
}
