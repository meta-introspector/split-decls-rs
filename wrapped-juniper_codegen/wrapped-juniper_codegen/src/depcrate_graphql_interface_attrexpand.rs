// Generated macro for expand (function)
macro_rules! Depcrate_graphql_interface_attrexpand {
() => {
// Module: crate::graphql_interface::attr
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[graphql_interface]` macro into generated code."] pub fn expand (attr_args : TokenStream , body : TokenStream) -> syn :: Result < TokenStream > { if let Ok (mut ast) = syn :: parse2 :: < syn :: ItemTrait > (body . clone ()) { let trait_attrs = parse :: attr :: unite (("graphql_interface" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_interface" , "graphql"] , ast . attrs) ; return expand_on_trait (trait_attrs , ast) ; } if let Ok (mut ast) = syn :: parse2 :: < syn :: DeriveInput > (body) { let trait_attrs = parse :: attr :: unite (("graphql_interface" , & attr_args) , & ast . attrs) ; ast . attrs = parse :: attr :: strip (["graphql_interface" , "graphql"] , ast . attrs) ; return expand_on_derive_input (trait_attrs , ast) ; } Err (syn :: Error :: new (Span :: call_site () , "#[graphql_interface] attribute is applicable to trait and struct \
         definitions only" ,)) }
};
}
