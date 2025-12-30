// Generated macro for http (module)
macro_rules! Depcratehttp {
() => {
// Module: crate
// Provides: {"http"}
// Dependencies: {}
# [doc = " Grammar rules of a simplified HTTP request parser"] # [allow (missing_docs)] pub mod http { # [doc = " HTTP parser."] # [derive (Parser)] # [grammar = "grammars/http.pest"] pub struct HttpParser ; }
};
}
