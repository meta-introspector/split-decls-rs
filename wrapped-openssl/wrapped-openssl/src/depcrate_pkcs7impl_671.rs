// Generated macro for impl_671 (impl)
macro_rules! Depcrate_pkcs7impl_671 {
() => {
// Module: crate::pkcs7
// Provides: {"impl_671"}
// Dependencies: {}
impl Pkcs7SignedRef { # [doc = " Get the stack of certificates from the PKCS7_SIGNED object"] pub fn certificates (& self) -> Option < & StackRef < X509 > > { unsafe { self . as_ptr () . as_ref () . and_then (| x | x . cert . as_mut ()) . and_then (| x | StackRef :: < X509 > :: from_const_ptr_opt (x)) } } }
};
}
