// Generated macro for impl_198 (impl)
macro_rules! Depcrate_repr_iterimpl_198 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_198"}
// Dependencies: {}
impl FromIterator < Box < str > > for Repr { fn from_iter < T : IntoIterator < Item = Box < str > > > (iter : T) -> Self { from_as_ref_str_iterator (iter . into_iter ()) } }
};
}
