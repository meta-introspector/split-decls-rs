// Generated macro for impl_133 (impl)
macro_rules! Depcrate_fallbackimpl_133 {
() => {
// Module: crate::fallback
// Provides: {"impl_133"}
// Dependencies: {}
impl FromIterator < TokenStream > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenStream > > (streams : I) -> Self { let mut v = RcVecBuilder :: new () ; for stream in streams { v . extend (stream . take_inner ()) ; } TokenStream { inner : v . build () } } }
};
}
