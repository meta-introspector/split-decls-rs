// Generated macro for impl_209 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_209 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_209"}
// Dependencies: {}
impl < T > From < RawIter < T > > for RawParIter < T > { fn from (it : RawIter < T >) -> Self { RawParIter { iter : it . iter } } }
};
}
