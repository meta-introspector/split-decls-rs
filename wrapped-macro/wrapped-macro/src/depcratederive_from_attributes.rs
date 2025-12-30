// Generated macro for derive_from_attributes (function)
macro_rules! Depcratederive_from_attributes {
() => {
// Module: crate
// Provides: {"derive_from_attributes"}
// Dependencies: {}
# [proc_macro_derive (FromAttributes , attributes (darling))] pub fn derive_from_attributes (input : TokenStream) -> TokenStream { derive :: from_attributes (& parse_macro_input ! (input)) . into () }
};
}
