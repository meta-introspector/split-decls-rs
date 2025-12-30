// Generated macro for derive_from_meta (function)
macro_rules! Depcratederive_from_meta {
() => {
// Module: crate
// Provides: {"derive_from_meta"}
// Dependencies: {}
# [proc_macro_derive (FromMeta , attributes (darling))] pub fn derive_from_meta (input : TokenStream) -> TokenStream { derive :: from_meta (& parse_macro_input ! (input)) . into () }
};
}
