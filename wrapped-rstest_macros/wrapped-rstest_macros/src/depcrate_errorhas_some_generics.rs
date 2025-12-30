// Generated macro for has_some_generics (function)
macro_rules! Depcrate_errorhas_some_generics {
() => {
// Module: crate::error
// Provides: {"has_some_generics"}
// Dependencies: {}
fn has_some_generics (test : & ItemFn) -> bool { ! test . sig . generics . params . is_empty () || SearchImpl :: function_has_some_impl (test) }
};
}
