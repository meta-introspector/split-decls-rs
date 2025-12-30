// Generated macro for build_nested_conditional (function)
macro_rules! Depcratebuild_nested_conditional {
() => {
// Module: crate
// Provides: {"build_nested_conditional"}
// Dependencies: {}
# [doc = " Generate nested if-else chain from variants"] fn build_nested_conditional (variants : Vec < (proc_macro2 :: TokenStream , proc_macro2 :: TokenStream) > ,) -> proc_macro2 :: TokenStream { if variants . is_empty () { return quote ! (compile_error ! ("No valid variants found")) ; } if variants . len () == 1 { return variants [0] . 1 . clone () ; } let mut result = variants . last () . unwrap () . 1 . clone () ; for (condition , tokens) in variants . iter () . rev () . skip (1) { result = quote ! { if # condition { # tokens } else { # result } } ; } quote ! { { # result } } }
};
}
