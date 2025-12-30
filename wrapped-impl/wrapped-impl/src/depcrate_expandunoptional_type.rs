// Generated macro for unoptional_type (function)
macro_rules! Depcrate_expandunoptional_type {
() => {
// Module: crate::expand
// Provides: {"unoptional_type"}
// Dependencies: {}
fn unoptional_type (ty : & Type) -> TokenStream { let unoptional = type_parameter_of_option (ty) . unwrap_or (ty) ; quote ! (# unoptional) }
};
}
