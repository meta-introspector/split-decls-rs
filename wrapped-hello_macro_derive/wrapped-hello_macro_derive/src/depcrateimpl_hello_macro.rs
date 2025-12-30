// Generated macro for impl_hello_macro (function)
macro_rules! Depcrateimpl_hello_macro {
() => {
// Module: crate
// Provides: {"impl_hello_macro"}
// Dependencies: {}
fn impl_hello_macro (ast : & syn :: DeriveInput) -> TokenStream { let name = & ast . ident ; let generated = quote ! { impl HelloMacro for # name { fn hello_macro () { println ! ("Hello, Macro! My name is {}!" , stringify ! (# name)) ; } } } ; generated . into () }
};
}
