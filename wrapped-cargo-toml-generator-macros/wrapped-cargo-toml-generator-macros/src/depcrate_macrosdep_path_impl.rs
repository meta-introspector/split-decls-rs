// Generated macro for dep_path_impl (function)
macro_rules! Depcrate_macrosdep_path_impl {
() => {
// Module: crate::macros
// Provides: {"dep_path_impl"}
// Dependencies: {}
pub fn dep_path_impl (input : TokenStream) -> TokenStream { let parsed : Punctuated < LitStr , Token ! [,] > = parse_macro_input ! (input with Punctuated :: parse_terminated) ; let name_lit = parsed . first () . expect ("Expected dependency name LitStr") . clone () ; let path_lit = parsed . last () . expect ("Expected dependency path LitStr") . clone () ; let name_ident = Ident :: new (& name_lit . value () , name_lit . span ()) ; quote ! { # name_ident = { path = # path_lit } } . into () }
};
}
