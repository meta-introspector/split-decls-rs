// Generated macro for impl_132 (impl)
macro_rules! Depcrate_fallbackimpl_132 {
() => {
// Module: crate::fallback
// Provides: {"impl_132"}
// Dependencies: {}
impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (tokens : I) -> Self { let mut stream = TokenStream :: new () ; stream . extend (tokens) ; stream } }
};
}
