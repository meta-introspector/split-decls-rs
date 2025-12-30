// Generated macro for derive_field (function)
macro_rules! Depcratederive_field {
() => {
// Module: crate
// Provides: {"derive_field"}
// Dependencies: {}
# [proc_macro_derive (FromField , attributes (darling))] pub fn derive_field (input : TokenStream) -> TokenStream { derive :: from_field (& parse_macro_input ! (input)) . into () }
};
}
