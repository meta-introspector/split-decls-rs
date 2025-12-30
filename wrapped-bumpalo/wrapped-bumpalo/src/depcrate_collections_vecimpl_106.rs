// Generated macro for impl_106 (impl)
macro_rules! Depcrate_collections_vecimpl_106 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'bump , T : 'bump > Extend < T > for Vec < 'bump , T > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { let iter = iter . into_iter () ; self . reserve (iter . size_hint () . 0) ; for t in iter { self . push (t) ; } } }
};
}
