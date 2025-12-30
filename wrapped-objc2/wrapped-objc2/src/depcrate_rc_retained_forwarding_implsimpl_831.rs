// Generated macro for impl_831 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_831 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_831"}
// Dependencies: {}
impl < 'a , A , T : ? Sized > Extend < A > for & 'a Retained < T > where & 'a T : Extend < A > , { # [inline] fn extend < I : IntoIterator < Item = A > > (& mut self , iter : I) { (& * * * self) . extend (iter) } }
};
}
