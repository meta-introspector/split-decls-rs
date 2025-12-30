// Generated macro for impl_134 (impl)
macro_rules! Depcrate_fallbackimpl_134 {
() => {
// Module: crate::fallback
// Provides: {"impl_134"}
// Dependencies: {}
impl Extend < TokenTree > for TokenStream { fn extend < I : IntoIterator < Item = TokenTree > > (& mut self , tokens : I) { let mut vec = self . inner . make_mut () ; tokens . into_iter () . for_each (| token | push_token_from_proc_macro (vec . as_mut () , token)) ; } }
};
}
