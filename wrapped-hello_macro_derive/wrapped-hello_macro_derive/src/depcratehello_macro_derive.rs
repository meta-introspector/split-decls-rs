// Generated macro for hello_macro_derive (function)
macro_rules! Depcratehello_macro_derive {
() => {
// Module: crate
// Provides: {"hello_macro_derive"}
// Dependencies: {}
# [proc_macro_derive (HelloMacro)] pub fn hello_macro_derive (input : TokenStream) -> TokenStream { let ast = syn :: parse (input) . unwrap () ; impl_hello_macro (& ast) }
};
}
