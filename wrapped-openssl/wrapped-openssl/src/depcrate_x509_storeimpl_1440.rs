// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_x509_storeimpl_1440 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1440"}
// Dependencies: {}
impl X509Lookup < HashDir > { # [doc = " Lookup method that loads certificates and CRLs on demand and caches"] # [doc = " them in memory once they are loaded. It also checks for newer CRLs upon"] # [doc = " each lookup, so that newer CRLs are used as soon as they appear in the"] # [doc = " directory."] # [corresponds (X509_LOOKUP_hash_dir)] pub fn hash_dir () -> & 'static X509LookupMethodRef < HashDir > { unsafe { X509LookupMethodRef :: from_const_ptr (ffi :: X509_LOOKUP_hash_dir ()) } } }
};
}
