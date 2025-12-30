// Generated macro for impl_201 (impl)
macro_rules! Depcrate_repr_iterimpl_201 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'a > FromIterator < Cow < 'a , str > > for Repr { fn from_iter < T : IntoIterator < Item = Cow < 'a , str > > > (iter : T) -> Self { from_as_ref_str_iterator (iter . into_iter ()) } }
};
}
