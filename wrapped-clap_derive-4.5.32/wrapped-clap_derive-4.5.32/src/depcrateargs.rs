// Generated macro for args (function)
macro_rules! Depcrateargs {
() => {
// Module: crate
// Provides: {"args"}
// Dependencies: {}
# [doc = " Generates the `Args` impl."] # [proc_macro_derive (Args , attributes (clap , command , arg , group))] pub fn args (input : TokenStream) -> TokenStream { let input : DeriveInput = parse_macro_input ! (input) ; derives :: derive_args (& input) . unwrap_or_else (| err | { let dummy = dummies :: args (& input . ident) ; to_compile_error (err , dummy) }) . into () }
};
}
