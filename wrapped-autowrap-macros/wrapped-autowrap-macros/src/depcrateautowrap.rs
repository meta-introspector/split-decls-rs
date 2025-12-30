// Generated macro for autowrap (function)
macro_rules! Depcrateautowrap {
() => {
// Module: crate
// Provides: {"autowrap"}
// Dependencies: {}
# [proc_macro] pub fn autowrap (input : TokenStream) -> TokenStream { let code = parse_macro_input ! (input as LitStr) . value () ; if let Ok (wrapped) = try_compile_and_wrap (& code) { return wrapped . parse () . unwrap () ; } let output = quote ! { { prelude ! { use std ::*; } mkdecl ! { # code } } } ; output . into () }
};
}
