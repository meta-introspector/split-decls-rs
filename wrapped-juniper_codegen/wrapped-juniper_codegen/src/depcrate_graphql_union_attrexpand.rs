// Generated macro for expand (function)
macro_rules! Depcrate_graphql_union_attrexpand {
() => {
// Module: crate::graphql_union::attr
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[graphql_union]` macro into generated code."] pub fn expand (attr_args : TokenStream , body : TokenStream) -> syn :: Result < TokenStream > { if let Ok (mut ast) = syn :: parse2 :: < syn :: ItemTrait > (body) { let trait_attrs = parse :: attr :: unite (("graphql_union" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_union" , "graphql"] , ast . attrs) ; return expand_on_trait (trait_attrs , ast) ; } Err (syn :: Error :: new (Span :: call_site () , "#[graphql_union] attribute is applicable to trait definitions only" ,)) }
};
}
