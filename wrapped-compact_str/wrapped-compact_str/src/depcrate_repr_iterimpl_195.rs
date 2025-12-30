// Generated macro for impl_195 (impl)
macro_rules! Depcrate_repr_iterimpl_195 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a char > for Repr { fn from_iter < T : IntoIterator < Item = & 'a char > > (iter : T) -> Self { iter . into_iter () . copied () . collect () } }
};
}
