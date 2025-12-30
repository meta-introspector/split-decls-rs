// Generated macro for impl_146 (impl)
macro_rules! Depcrate_equivalentimpl_146 {
() => {
// Module: crate::equivalent
// Provides: {"impl_146"}
// Dependencies: {}
impl < K : ? Sized , Q : ? Sized > Equivalent < Q > for K where K : Borrow < Q > , Q : Eq , { # [inline] fn equivalent (& self , key : & Q) -> bool { PartialEq :: eq (self . borrow () , key) } }
};
}
