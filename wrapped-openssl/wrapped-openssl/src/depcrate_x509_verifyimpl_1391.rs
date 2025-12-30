// Generated macro for impl_1391 (impl)
macro_rules! Depcrate_x509_verifyimpl_1391 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1391"}
// Dependencies: {}
impl X509VerifyParam { # [doc = " Create an X509VerifyParam"] # [corresponds (X509_VERIFY_PARAM_new)] pub fn new () -> Result < X509VerifyParam , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: X509_VERIFY_PARAM_new ()) . map (X509VerifyParam) } } }
};
}
