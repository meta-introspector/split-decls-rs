// Generated macro for impl_254 (impl)
macro_rules! Depcrateimpl_254 {
() => {
// Module: crate
// Provides: {"impl_254"}
// Dependencies: {}
impl Extend < TokenStream > for TokenStream { fn extend < I : IntoIterator < Item = TokenStream > > (& mut self , streams : I) { self . inner . extend (streams . into_iter () . map (| stream | stream . inner)) ; } }
};
}
