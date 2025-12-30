// Generated macro for impl_131 (impl)
macro_rules! Depcrate_fallbackimpl_131 {
() => {
// Module: crate::fallback
// Provides: {"impl_131"}
// Dependencies: {}
impl From < TokenTree > for TokenStream { fn from (tree : TokenTree) -> Self { let mut stream = RcVecBuilder :: new () ; push_token_from_proc_macro (stream . as_mut () , tree) ; TokenStream { inner : stream . build () , } } }
};
}
