// Generated macro for expand (function)
macro_rules! Depcrateexpand {
() => {
// Module: crate
// Provides: {"expand"}
// Dependencies: {}
fn expand (input : TokenStream , mode : Macro) -> TokenStream { match try_expand (input , mode) { Ok (tokens) => tokens , Err (err) => err . to_compile_error () , } }
};
}
