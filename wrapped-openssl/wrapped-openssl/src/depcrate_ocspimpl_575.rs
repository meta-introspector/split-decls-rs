// Generated macro for impl_575 (impl)
macro_rules! Depcrate_ocspimpl_575 {
() => {
// Module: crate::ocsp
// Provides: {"impl_575"}
// Dependencies: {}
impl OcspResponseStatus { pub const SUCCESSFUL : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_SUCCESSFUL) ; pub const MALFORMED_REQUEST : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_MALFORMEDREQUEST) ; pub const INTERNAL_ERROR : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_INTERNALERROR) ; pub const TRY_LATER : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_TRYLATER) ; pub const SIG_REQUIRED : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_SIGREQUIRED) ; pub const UNAUTHORIZED : OcspResponseStatus = OcspResponseStatus (ffi :: OCSP_RESPONSE_STATUS_UNAUTHORIZED) ; pub fn from_raw (raw : c_int) -> OcspResponseStatus { OcspResponseStatus (raw) } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
