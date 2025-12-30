// Generated macro for derive_from_input (function)
macro_rules! Depcratederive_from_input {
() => {
// Module: crate
// Provides: {"derive_from_input"}
// Dependencies: {}
# [proc_macro_derive (FromDeriveInput , attributes (darling))] pub fn derive_from_input (input : TokenStream) -> TokenStream { derive :: from_derive_input (& parse_macro_input ! (input)) . into () }
};
}
