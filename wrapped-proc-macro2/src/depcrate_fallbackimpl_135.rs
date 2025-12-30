// Generated macro for impl_135 (impl)
macro_rules! Depcrate_fallbackimpl_135 {
() => {
// Module: crate::fallback
// Provides: {"impl_135"}
// Dependencies: {}
impl Extend < TokenStream > for TokenStream { fn extend < I : IntoIterator < Item = TokenStream > > (& mut self , streams : I) { self . inner . make_mut () . extend (streams . into_iter () . flatten ()) ; } }
};
}
