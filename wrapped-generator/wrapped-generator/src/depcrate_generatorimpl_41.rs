// Generated macro for impl_41 (impl)
macro_rules! Depcrate_generatorimpl_41 {
() => {
// Module: crate::generator
// Provides: {"impl_41"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for QuoteOption < T > { fn to_tokens (& self , tokens : & mut TokenStream) { let option = option_type () ; tokens . append_all (match self . 0 { Some (ref t) => quote ! { # option :: Some (# t) } , None => quote ! { # option :: None } , }) ; } }
};
}
