// Generated macro for impl_57 (impl)
macro_rules! Depcrate_astimpl_57 {
() => {
// Module: crate::ast
// Provides: {"impl_57"}
// Dependencies: {}
impl < A > FromIterator < AList < A > > for AttrList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = AList < A > > , { Self { elems : iter . into_iter () . map (| u | u . into_iter () . collect ()) . collect () , } } }
};
}
