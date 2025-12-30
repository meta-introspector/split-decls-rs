// Generated macro for impl_541 (impl)
macro_rules! Depcrate_tokenstreamimpl_541 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_541"}
// Dependencies: {}
impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (iter : I) -> Self { TokenStream :: new (iter . into_iter () . collect :: < Vec < TokenTree > > ()) } }
};
}
