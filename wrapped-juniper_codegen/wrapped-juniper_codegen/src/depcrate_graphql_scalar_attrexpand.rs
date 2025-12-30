// Generated macro for expand (function)
macro_rules! Depcrate_graphql_scalar_attrexpand {
() => {
// Module: crate::graphql_scalar::attr
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[graphql_scalar]` macro into generated code."] pub (crate) fn expand (attr_args : TokenStream , body : TokenStream) -> syn :: Result < TokenStream > { if let Ok (mut ast) = syn :: parse2 :: < syn :: ItemType > (body . clone ()) { let attrs = parse :: attr :: unite (("graphql_scalar" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_scalar" , "graphql"] , ast . attrs) ; return expand_on_type_alias (attrs , ast) ; } else if let Ok (mut ast) = syn :: parse2 :: < syn :: DeriveInput > (body) { let attrs = parse :: attr :: unite (("graphql_scalar" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_scalar" , "graphql"] , ast . attrs) ; return expand_on_derive_input (attrs , ast) ; } Err (syn :: Error :: new (Span :: call_site () , "#[graphql_scalar] attribute is applicable to type aliases, structs, \
         enums and unions only" ,)) }
};
}
