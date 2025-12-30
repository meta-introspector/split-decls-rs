// Generated macro for attr_error (function)
macro_rules! Depcrateattr_error {
() => {
// Module: crate
// Provides: {"attr_error"}
// Dependencies: {}
# [proc_macro_attribute] pub fn attr_error (args : TokenStream , item : TokenStream) -> TokenStream { format ! ("compile_error!(\"#[attr_error({})] {}\");" , args , item) . parse () . unwrap () }
};
}
