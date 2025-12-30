// Generated macro for impl_587 (impl)
macro_rules! Depcrate_ocspimpl_587 {
() => {
// Module: crate::ocsp
// Provides: {"impl_587"}
// Dependencies: {}
impl OcspResponse { # [doc = " Creates an OCSP response from the status and optional body."] # [doc = ""] # [doc = " A body should only be provided if `status` is `RESPONSE_STATUS_SUCCESSFUL`."] # [corresponds (OCSP_response_create)] pub fn create (status : OcspResponseStatus , body : Option < & OcspBasicResponseRef > ,) -> Result < OcspResponse , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: OCSP_response_create (status . as_raw () , body . map (| r | r . as_ptr ()) . unwrap_or (ptr :: null_mut ()) ,)) . map (OcspResponse) } } from_der ! { # [doc = " Deserializes a DER-encoded OCSP response."] # [corresponds (d2i_OCSP_RESPONSE)] from_der , OcspResponse , ffi :: d2i_OCSP_RESPONSE } }
};
}
