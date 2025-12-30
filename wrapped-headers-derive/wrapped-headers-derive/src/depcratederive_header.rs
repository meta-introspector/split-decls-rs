// Generated macro for derive_header (function)
macro_rules! Depcratederive_header {
() => {
// Module: crate
// Provides: {"derive_header"}
// Dependencies: {}
# [proc_macro_derive (Header , attributes (header))] pub fn derive_header (input : TokenStream) -> TokenStream { let ast = syn :: parse (input) . unwrap () ; impl_header (& ast) . into () }
};
}
