// Generated macro for derive_type_param (function)
macro_rules! Depcratederive_type_param {
() => {
// Module: crate
// Provides: {"derive_type_param"}
// Dependencies: {}
# [proc_macro_derive (FromTypeParam , attributes (darling))] pub fn derive_type_param (input : TokenStream) -> TokenStream { derive :: from_type_param (& parse_macro_input ! (input)) . into () }
};
}
