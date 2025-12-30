// Generated macro for expand (function)
macro_rules! Depcrate_graphql_union_deriveexpand {
() => {
// Module: crate::graphql_union::derive
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[derive(GraphQLUnion)]` macro into generated code."] pub fn expand (input : TokenStream) -> syn :: Result < TokenStream > { let ast = syn :: parse2 :: < syn :: DeriveInput > (input) . unwrap_or_abort () ; match & ast . data { Data :: Enum (_) => expand_enum (ast) , Data :: Struct (_) => expand_struct (ast) , _ => Err (ERR . custom_error (ast . span () , "can only be derived for enums and structs")) , } . map (ToTokens :: into_token_stream) }
};
}
