// Generated macro for expand (function)
macro_rules! Depcrate_fallbackexpand {
() => {
// Module: crate::fallback
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand (input : & DeriveInput , error : syn :: Error) -> TokenStream { let ty = call_site_ident (& input . ident) ; let (impl_generics , ty_generics , where_clause) = input . generics . split_for_impl () ; let error = error . to_compile_error () ; quote ! { # error # [allow (unused_qualifications)] # [automatically_derived] impl # impl_generics :: thiserror ::# private :: Error for # ty # ty_generics # where_clause where for <'workaround > # ty # ty_generics : :: core :: fmt :: Debug , { } # [allow (unused_qualifications)] # [automatically_derived] impl # impl_generics :: core :: fmt :: Display for # ty # ty_generics # where_clause { fn fmt (& self , __formatter : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { :: core :: unreachable ! () } } } }
};
}
