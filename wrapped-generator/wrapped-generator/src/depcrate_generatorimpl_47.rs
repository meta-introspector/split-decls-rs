// Generated macro for impl_47 (impl)
macro_rules! Depcrate_generatorimpl_47 {
() => {
// Module: crate::generator
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for QuoteOption < T > { fn to_tokens (& self , tokens : & mut TokenStream) { let option = option_type () ; tokens . append_all (match self . 0 { Some (ref t) => quote ! { # option :: Some (# t) } , None => quote ! { # option :: None } , }) ; } }
};
}
