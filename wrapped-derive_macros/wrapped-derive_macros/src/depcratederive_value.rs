// Generated macro for derive_value (function)
macro_rules! Depcratederive_value {
() => {
// Module: crate
// Provides: {"derive_value"}
// Dependencies: {}
# [proc_macro_derive (Value , attributes (sval))] pub fn derive_value (input : TokenStream) -> TokenStream { TokenStream :: from (derive :: derive (parse_macro_input ! (input as DeriveInput))) }
};
}
