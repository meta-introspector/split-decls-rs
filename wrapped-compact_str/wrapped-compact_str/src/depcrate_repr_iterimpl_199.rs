// Generated macro for impl_199 (impl)
macro_rules! Depcrate_repr_iterimpl_199 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_199"}
// Dependencies: {}
impl FromIterator < String > for Repr { fn from_iter < T : IntoIterator < Item = String > > (iter : T) -> Self { from_as_ref_str_iterator (iter . into_iter ()) } }
};
}
