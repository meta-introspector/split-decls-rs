// Generated macro for impl_200 (impl)
macro_rules! Depcrate_repr_iterimpl_200 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_200"}
// Dependencies: {}
impl FromIterator < CompactString > for Repr { fn from_iter < T : IntoIterator < Item = CompactString > > (iter : T) -> Self { from_as_ref_str_iterator (iter . into_iter ()) } }
};
}
