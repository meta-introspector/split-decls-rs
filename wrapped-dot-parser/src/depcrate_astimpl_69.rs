// Generated macro for impl_69 (impl)
macro_rules! Depcrate_astimpl_69 {
() => {
// Module: crate::ast
// Provides: {"impl_69"}
// Dependencies: {}
impl < A > From < AttrList < A > > for AList < A > { fn from (attr : AttrList < A >) -> Self { attr . into_iter () . flatten () . collect () } }
};
}
