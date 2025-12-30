// Generated macro for expand (function)
macro_rules! Depcrate_graphql_scalar_deriveexpand {
() => {
// Module: crate::graphql_scalar::derive
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[derive(GraphQLScalar)]` macro into generated code."] pub fn expand (input : TokenStream) -> syn :: Result < TokenStream > { let ast = syn :: parse2 :: < syn :: DeriveInput > (input) ? ; let attr = Attr :: from_attrs ("graphql" , & ast . attrs) ? ; let methods = parse_derived_methods (& ast , & attr) ? ; let scalar = scalar :: Type :: parse (attr . scalar . as_deref () , & ast . generics) ; Ok (Definition { ty : TypeOrIdent :: Ident (ast . ident . clone ()) , where_clause : attr . where_clause . map_or_else (Vec :: new , | cl | cl . into_inner ()) , generics : ast . generics . clone () , methods , name : attr . name . map (SpanContainer :: into_inner) . unwrap_or_else (| | ast . ident . to_string ()) , description : attr . description . map (SpanContainer :: into_inner) , specified_by_url : attr . specified_by_url . map (SpanContainer :: into_inner) , scalar , } . to_token_stream ()) }
};
}
