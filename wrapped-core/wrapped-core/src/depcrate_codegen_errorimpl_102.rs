// Generated macro for impl_102 (impl)
macro_rules! Depcrate_codegen_errorimpl_102 {
() => {
// Module: crate::codegen::error
// Provides: {"impl_102"}
// Dependencies: {}
impl ToTokens for ErrorDeclaration { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (quote ! { let mut __errors = :: darling :: Error :: accumulator () ; }) } }
};
}
