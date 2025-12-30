// Generated macro for impl_577 (impl)
macro_rules! Depcrate_ocspimpl_577 {
() => {
// Module: crate::ocsp
// Provides: {"impl_577"}
// Dependencies: {}
impl OcspCertStatus { pub const GOOD : OcspCertStatus = OcspCertStatus (ffi :: V_OCSP_CERTSTATUS_GOOD) ; pub const REVOKED : OcspCertStatus = OcspCertStatus (ffi :: V_OCSP_CERTSTATUS_REVOKED) ; pub const UNKNOWN : OcspCertStatus = OcspCertStatus (ffi :: V_OCSP_CERTSTATUS_UNKNOWN) ; pub fn from_raw (raw : c_int) -> OcspCertStatus { OcspCertStatus (raw) } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
