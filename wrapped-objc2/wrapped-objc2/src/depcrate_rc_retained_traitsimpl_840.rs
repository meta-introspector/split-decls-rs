// Generated macro for impl_840 (impl)
macro_rules! Depcrate_rc_retained_traitsimpl_840 {
() => {
// Module: crate::rc::retained_traits
// Provides: {"impl_840"}
// Dependencies: {}
impl < T , U : RetainedFromIterator < T > > FromIterator < T > for Retained < U > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { U :: retained_from_iter (iter) } }
};
}
