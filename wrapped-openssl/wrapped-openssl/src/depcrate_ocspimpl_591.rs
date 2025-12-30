// Generated macro for impl_591 (impl)
macro_rules! Depcrate_ocspimpl_591 {
() => {
// Module: crate::ocsp
// Provides: {"impl_591"}
// Dependencies: {}
impl OcspRequestRef { to_der ! { # [doc = " Serializes the request to its standard DER encoding."] # [corresponds (i2d_OCSP_REQUEST)] to_der , ffi :: i2d_OCSP_REQUEST } # [corresponds (OCSP_request_add0_id)] pub fn add_id (& mut self , id : OcspCertId) -> Result < & mut OcspOneReqRef , ErrorStack > { unsafe { let ptr = cvt_p (ffi :: OCSP_request_add0_id (self . as_ptr () , id . as_ptr ())) ? ; mem :: forget (id) ; Ok (OcspOneReqRef :: from_ptr_mut (ptr)) } } }
};
}
