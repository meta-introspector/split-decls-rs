// Generated macro for impl_256 (impl)
macro_rules! Depcrateimpl_256 {
() => {
// Module: crate
// Provides: {"impl_256"}
// Dependencies: {}
impl FromIterator < TokenStream > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenStream > > (streams : I) -> Self { TokenStream :: _new (streams . into_iter () . map (| i | i . inner) . collect ()) } }
};
}
