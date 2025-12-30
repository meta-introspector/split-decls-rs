// Generated macro for impl_1539 (impl)
macro_rules! Depcrate_x509impl_1539 {
() => {
// Module: crate::x509
// Provides: {"impl_1539"}
// Dependencies: {}
impl Ord for X509Ref { fn cmp (& self , other : & Self) -> cmp :: Ordering { let cmp = unsafe { ffi :: X509_cmp (self . as_ptr () , other . as_ptr ()) } ; cmp . cmp (& 0) } }
};
}
