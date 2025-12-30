// Generated macro for impl_590 (impl)
macro_rules! Depcrate_ocspimpl_590 {
() => {
// Module: crate::ocsp
// Provides: {"impl_590"}
// Dependencies: {}
impl OcspRequest { # [corresponds (OCSP_REQUEST_new)] pub fn new () -> Result < OcspRequest , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: OCSP_REQUEST_new ()) . map (OcspRequest) } } from_der ! { # [doc = " Deserializes a DER-encoded OCSP request."] # [corresponds (d2i_OCSP_REQUEST)] from_der , OcspRequest , ffi :: d2i_OCSP_REQUEST } }
};
}
