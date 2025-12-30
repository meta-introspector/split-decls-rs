// Generated macro for impl_70 (impl)
macro_rules! Depcrate_astimpl_70 {
() => {
// Module: crate::ast
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'a , A > From < & 'a AttrList < A > > for AList < & 'a A > { fn from (attr : & 'a AttrList < A >) -> Self { attr . into_iter () . flatten () . collect () } }
};
}
