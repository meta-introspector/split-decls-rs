// Generated macro for err_variant_with_fields (function)
macro_rules! Depcrate_graphql_enum_deriveerr_variant_with_fields {
() => {
// Module: crate::graphql_enum::derive
// Provides: {"err_variant_with_fields"}
// Dependencies: {}
# [doc = " Emits \"no fields allowed for non-ignored variants\" [`syn::Error`] pointing"] # [doc = " to the given `span`."] pub fn err_variant_with_fields < T , S : Spanned > (span : & S) -> Option < T > { ERR . emit_custom (span . span () , "no fields allowed for non-ignored variants") ; None }
};
}
