// Generated macro for dep_version_impl (function)
macro_rules! Depcrate_macrosdep_version_impl {
() => {
// Module: crate::macros
// Provides: {"dep_version_impl"}
// Dependencies: {}
pub fn dep_version_impl (input : TokenStream) -> TokenStream { let parsed : Punctuated < LitStr , Token ! [,] > = parse_macro_input ! (input with Punctuated :: parse_terminated) ; let name_lit = parsed . first () . expect ("Expected dependency name LitStr") . clone () ; let version_lit = parsed . last () . expect ("Expected dependency version LitStr") . clone () ; let name_ident = Ident :: new (& name_lit . value () , name_lit . span ()) ; quote ! { # name_ident = # version_lit } . into () }
};
}
