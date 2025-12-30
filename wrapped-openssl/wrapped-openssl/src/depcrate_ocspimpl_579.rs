// Generated macro for impl_579 (impl)
macro_rules! Depcrate_ocspimpl_579 {
() => {
// Module: crate::ocsp
// Provides: {"impl_579"}
// Dependencies: {}
impl OcspRevokedStatus { pub const NO_STATUS : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_NOSTATUS) ; pub const UNSPECIFIED : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_UNSPECIFIED) ; pub const KEY_COMPROMISE : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_KEYCOMPROMISE) ; pub const CA_COMPROMISE : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_CACOMPROMISE) ; pub const AFFILIATION_CHANGED : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_AFFILIATIONCHANGED) ; pub const STATUS_SUPERSEDED : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_SUPERSEDED) ; pub const STATUS_CESSATION_OF_OPERATION : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_CESSATIONOFOPERATION) ; pub const STATUS_CERTIFICATE_HOLD : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_CERTIFICATEHOLD) ; pub const REMOVE_FROM_CRL : OcspRevokedStatus = OcspRevokedStatus (ffi :: OCSP_REVOKED_STATUS_REMOVEFROMCRL) ; pub fn from_raw (raw : c_int) -> OcspRevokedStatus { OcspRevokedStatus (raw) } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
