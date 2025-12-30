// Generated macro for impl_588 (impl)
macro_rules! Depcrate_ocspimpl_588 {
() => {
// Module: crate::ocsp
// Provides: {"impl_588"}
// Dependencies: {}
impl OcspResponseRef { to_der ! { # [doc = " Serializes the response to its standard DER encoding."] # [corresponds (i2d_OCSP_RESPONSE)] to_der , ffi :: i2d_OCSP_RESPONSE } # [doc = " Returns the status of the response."] # [corresponds (OCSP_response_status)] pub fn status (& self) -> OcspResponseStatus { unsafe { OcspResponseStatus (ffi :: OCSP_response_status (self . as_ptr ())) } } # [doc = " Returns the basic response."] # [doc = ""] # [doc = " This will only succeed if `status()` returns `RESPONSE_STATUS_SUCCESSFUL`."] # [corresponds (OCSP_response_get1_basic)] pub fn basic (& self) -> Result < OcspBasicResponse , ErrorStack > { unsafe { cvt_p (ffi :: OCSP_response_get1_basic (self . as_ptr ())) . map (OcspBasicResponse) } } }
};
}
