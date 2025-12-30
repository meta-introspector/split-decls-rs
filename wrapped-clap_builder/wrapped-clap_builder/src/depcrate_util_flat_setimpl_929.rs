// Generated macro for impl_929 (impl)
macro_rules! Depcrate_util_flat_setimpl_929 {
() => {
// Module: crate::util::flat_set
// Provides: {"impl_929"}
// Dependencies: {}
impl < T : PartialEq + Eq > Extend < T > for FlatSet < T > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for value in iter { self . insert (value) ; } } }
};
}
