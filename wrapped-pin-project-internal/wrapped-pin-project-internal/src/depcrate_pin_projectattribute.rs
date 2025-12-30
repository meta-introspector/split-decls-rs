// Generated macro for attribute (function)
macro_rules! Depcrate_pin_projectattribute {
() => {
// Module: crate::pin_project
// Provides: {"attribute"}
// Dependencies: {}
pub (crate) fn attribute (args : & TokenStream , input : TokenStream) -> TokenStream { attribute :: parse_attribute (args , input) . unwrap_or_else (Error :: into_compile_error) }
};
}
