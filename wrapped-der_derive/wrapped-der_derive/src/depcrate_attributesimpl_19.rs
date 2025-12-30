// Generated macro for impl_19 (impl)
macro_rules! Depcrate_attributesimpl_19 {
() => {
// Module: crate::attributes
// Provides: {"impl_19"}
// Dependencies: {}
impl ToTokens for ErrorType { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: Der => { let err = quote ! { :: der :: Error } ; err . to_tokens (tokens) } Self :: Custom (path) => path . to_tokens (tokens) , } } }
};
}
