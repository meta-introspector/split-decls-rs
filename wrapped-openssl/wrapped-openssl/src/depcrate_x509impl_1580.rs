// Generated macro for impl_1580 (impl)
macro_rules! Depcrate_x509impl_1580 {
() => {
// Module: crate::x509
// Provides: {"impl_1580"}
// Dependencies: {}
# [allow (missing_docs)] impl CrlReason { pub const UNSPECIFIED : CrlReason = CrlReason (ffi :: CRL_REASON_UNSPECIFIED) ; pub const KEY_COMPROMISE : CrlReason = CrlReason (ffi :: CRL_REASON_KEY_COMPROMISE) ; pub const CA_COMPROMISE : CrlReason = CrlReason (ffi :: CRL_REASON_CA_COMPROMISE) ; pub const AFFILIATION_CHANGED : CrlReason = CrlReason (ffi :: CRL_REASON_AFFILIATION_CHANGED) ; pub const SUPERSEDED : CrlReason = CrlReason (ffi :: CRL_REASON_SUPERSEDED) ; pub const CESSATION_OF_OPERATION : CrlReason = CrlReason (ffi :: CRL_REASON_CESSATION_OF_OPERATION) ; pub const CERTIFICATE_HOLD : CrlReason = CrlReason (ffi :: CRL_REASON_CERTIFICATE_HOLD) ; pub const REMOVE_FROM_CRL : CrlReason = CrlReason (ffi :: CRL_REASON_REMOVE_FROM_CRL) ; pub const PRIVILEGE_WITHDRAWN : CrlReason = CrlReason (ffi :: CRL_REASON_PRIVILEGE_WITHDRAWN) ; pub const AA_COMPROMISE : CrlReason = CrlReason (ffi :: CRL_REASON_AA_COMPROMISE) ; # [doc = " Constructs an `CrlReason` from a raw OpenSSL value."] pub const fn from_raw (value : c_int) -> Self { CrlReason (value) } # [doc = " Returns the raw OpenSSL value represented by this type."] pub const fn as_raw (& self) -> c_int { self . 0 } }
};
}
