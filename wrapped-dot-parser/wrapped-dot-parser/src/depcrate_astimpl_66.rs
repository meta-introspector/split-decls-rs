// Generated macro for impl_66 (impl)
macro_rules! Depcrate_astimpl_66 {
() => {
// Module: crate::ast
// Provides: {"impl_66"}
// Dependencies: {}
impl < A > FromIterator < A > for AList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = A > , { Self { elems : iter . into_iter () . collect () , } } }
};
}
