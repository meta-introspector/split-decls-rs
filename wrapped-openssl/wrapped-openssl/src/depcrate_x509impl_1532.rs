// Generated macro for impl_1532 (impl)
macro_rules! Depcrate_x509impl_1532 {
() => {
// Module: crate::x509
// Provides: {"impl_1532"}
// Dependencies: {}
impl X509StoreContext { # [doc = " Returns the index which can be used to obtain a reference to the `Ssl` associated with a"] # [doc = " context."] # [corresponds (SSL_get_ex_data_X509_STORE_CTX_idx)] pub fn ssl_idx () -> Result < Index < X509StoreContext , SslRef > , ErrorStack > { unsafe { cvt_n (ffi :: SSL_get_ex_data_X509_STORE_CTX_idx ()) . map (| idx | Index :: from_raw (idx)) } } # [doc = " Creates a new `X509StoreContext` instance."] # [corresponds (X509_STORE_CTX_new)] pub fn new () -> Result < X509StoreContext , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: X509_STORE_CTX_new ()) . map (X509StoreContext) } } }
};
}
