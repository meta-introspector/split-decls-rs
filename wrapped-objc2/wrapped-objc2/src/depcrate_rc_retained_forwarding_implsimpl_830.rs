// Generated macro for impl_830 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_830 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_830"}
// Dependencies: {}
impl < A , T : ? Sized > Extend < A > for Retained < T > where for < 'a > & 'a T : Extend < A > , { # [inline] fn extend < I : IntoIterator < Item = A > > (& mut self , iter : I) { (& * * self) . extend (iter) } }
};
}
