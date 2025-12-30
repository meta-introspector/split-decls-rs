// Generated macro for expand (function)
macro_rules! Depcrate_graphql_subscription_attrexpand {
() => {
// Module: crate::graphql_subscription::attr
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[graphql_subscription]` macro into generated code."] pub fn expand (attr_args : TokenStream , body : TokenStream) -> syn :: Result < TokenStream > { if let Ok (mut ast) = syn :: parse2 :: < syn :: ItemImpl > (body) { if ast . trait_ . is_none () { let impl_attrs = parse :: attr :: unite (("graphql_subscription" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_subscription" , "graphql"] , ast . attrs) ; return expand_on_impl :: < Subscription > (Attr :: from_attrs (["graphql_subscription" , "graphql"] , & impl_attrs) ? , ast ,) ; } } Err (syn :: Error :: new (Span :: call_site () , "#[graphql_subscription] attribute is applicable to non-trait `impl` blocks only" ,)) }
};
}
