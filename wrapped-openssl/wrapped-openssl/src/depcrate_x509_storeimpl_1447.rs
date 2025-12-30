// Generated macro for impl_1447 (impl)
macro_rules! Depcrate_x509_storeimpl_1447 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1447"}
// Dependencies: {}
impl X509StoreRef { # [doc = " Get a reference to the cache of certificates in this store."] # [doc = ""] # [doc = " This method is deprecated. It is **unsound** and will be removed in a"] # [doc = " future version of rust-openssl. `X509StoreRef::all_certificates`"] # [doc = " should be used instead."] # [deprecated (note = "This method is unsound, and will be removed in a future version of rust-openssl. X509StoreRef::all_certificates should be used instead.")] # [corresponds (X509_STORE_get0_objects)] pub fn objects (& self) -> & StackRef < X509Object > { unsafe { StackRef :: from_ptr (X509_STORE_get0_objects (self . as_ptr ())) } } # [doc = " Returns a stack of all the certificates in this store."] # [corresponds (X509_STORE_get1_all_certs)] # [cfg (ossl300)] pub fn all_certificates (& self) -> Stack < X509 > { unsafe { Stack :: from_ptr (ffi :: X509_STORE_get1_all_certs (self . as_ptr ())) } } }
};
}
