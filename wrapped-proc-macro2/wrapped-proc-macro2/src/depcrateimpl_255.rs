// Generated macro for impl_255 (impl)
macro_rules! Depcrateimpl_255 {
() => {
// Module: crate
// Provides: {"impl_255"}
// Dependencies: {}
# [doc = " Collects a number of token trees into a single stream."] impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (tokens : I) -> Self { TokenStream :: _new (tokens . into_iter () . collect ()) } }
};
}
