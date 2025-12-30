// Generated macro for impl_1617 (impl)
macro_rules! Depcrate_x509impl_1617 {
() => {
// Module: crate::x509
// Provides: {"impl_1617"}
// Dependencies: {}
impl X509ObjectRef { pub fn x509 (& self) -> Option < & X509Ref > { unsafe { let ptr = X509_OBJECT_get0_X509 (self . as_ptr ()) ; X509Ref :: from_const_ptr_opt (ptr) } } }
};
}
