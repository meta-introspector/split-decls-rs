// Generated macro for derive_variant (function)
macro_rules! Depcratederive_variant {
() => {
// Module: crate
// Provides: {"derive_variant"}
// Dependencies: {}
# [proc_macro_derive (FromVariant , attributes (darling))] pub fn derive_variant (input : TokenStream) -> TokenStream { derive :: from_variant (& parse_macro_input ! (input)) . into () }
};
}
