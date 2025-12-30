// Generated macro for token_stream_with_error (function)
macro_rules! Depcratetoken_stream_with_error {
() => {
// Module: crate
// Provides: {"token_stream_with_error"}
// Dependencies: {}
pub (crate) fn token_stream_with_error (mut tokens : TokenStream , error : syn :: Error) -> TokenStream { tokens . extend (TokenStream :: from (error . into_compile_error ())) ; tokens }
};
}
