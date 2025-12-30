// Generated macro for TryToTokens (trait)
macro_rules! Depcrate_codegenTryToTokens {
() => {
// Module: crate::codegen
// Provides: {"TryToTokens"}
// Dependencies: {}
# [doc = " A trait for converting AST structs into Tokens and adding them to a TokenStream,"] # [doc = " or providing a diagnostic if conversion fails."] pub trait TryToTokens { # [doc = " Attempt to convert a `Self` into tokens and add it to the `TokenStream`"] fn try_to_tokens (& self , tokens : & mut TokenStream) -> Result < () , Diagnostic > ; # [doc = " Attempt to convert a `Self` into a new `TokenStream`"] fn try_to_token_stream (& self) -> Result < TokenStream , Diagnostic > { let mut tokens = TokenStream :: new () ; self . try_to_tokens (& mut tokens) ? ; Ok (tokens) } }
};
}
