// Generated macro for impl_197 (impl)
macro_rules! Depcrate_repr_iterimpl_197 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a str > for Repr { fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { from_as_ref_str_iterator (iter . into_iter ()) } }
};
}
