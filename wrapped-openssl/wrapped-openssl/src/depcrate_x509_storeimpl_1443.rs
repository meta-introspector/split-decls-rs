// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_x509_storeimpl_1443 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1443"}
// Dependencies: {}
impl X509Lookup < File > { # [doc = " Lookup method loads all the certificates or CRLs present in a file"] # [doc = " into memory at the time the file is added as a lookup source."] # [corresponds (X509_LOOKUP_file)] pub fn file () -> & 'static X509LookupMethodRef < File > { unsafe { X509LookupMethodRef :: from_const_ptr (ffi :: X509_LOOKUP_file ()) } } }
};
}
