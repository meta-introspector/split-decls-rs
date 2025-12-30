// Generated macro for expand (function)
macro_rules! Depcrate_graphql_object_deriveexpand {
() => {
// Module: crate::graphql_object::derive
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Expands `#[derive(GraphQLObject)]` macro into generated code."] pub fn expand (input : TokenStream) -> syn :: Result < TokenStream > { let ast = syn :: parse2 :: < syn :: DeriveInput > (input) . unwrap_or_abort () ; match & ast . data { syn :: Data :: Struct (_) => expand_struct (ast) , _ => Err (ERR . custom_error (ast . span () , "can only be derived for structs")) , } . map (ToTokens :: into_token_stream) }
};
}
