// Generated macro for impl_1436 (impl)
macro_rules! Depcrate_x509_storeimpl_1436 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1436"}
// Dependencies: {}
impl X509StoreBuilder { # [doc = " Returns a builder for a certificate store."] # [doc = ""] # [doc = " The store is initially empty."] # [corresponds (X509_STORE_new)] pub fn new () -> Result < X509StoreBuilder , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: X509_STORE_new ()) . map (X509StoreBuilder) } } # [doc = " Constructs the `X509Store`."] pub fn build (self) -> X509Store { let store = X509Store (self . 0) ; mem :: forget (self) ; store } }
};
}
