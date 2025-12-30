// Generated macro for map_invalid_certificate_error (function)
macro_rules! Depcrate_errormap_invalid_certificate_error {
() => {
// Module: crate::error
// Provides: {"map_invalid_certificate_error"}
// Dependencies: {}
fn map_invalid_certificate_error (err : CertificateError) -> rustls_result { use rustls_result :: * ; match err { CertificateError :: BadEncoding => CertEncodingBad , CertificateError :: Expired | CertificateError :: ExpiredContext { .. } => CertExpired , CertificateError :: NotValidYet | CertificateError :: NotValidYetContext { .. } => { CertNotYetValid } CertificateError :: Revoked => CertRevoked , CertificateError :: UnhandledCriticalExtension => CertUnhandledCriticalExtension , CertificateError :: UnknownIssuer => CertUnknownIssuer , CertificateError :: UnknownRevocationStatus => CertUnknownRevocationStatus , CertificateError :: ExpiredRevocationList | CertificateError :: ExpiredRevocationListContext { .. } => CertExpiredRevocationList , CertificateError :: BadSignature => CertBadSignature , CertificateError :: UnsupportedSignatureAlgorithmContext { .. } => { CertUnsupportedSignatureAlgorithm } CertificateError :: NotValidForName | CertificateError :: NotValidForNameContext { .. } => { CertNotValidForName } CertificateError :: InvalidPurpose | CertificateError :: InvalidPurposeContext { .. } => { CertInvalidPurpose } CertificateError :: ApplicationVerificationFailure => CertApplicationVerificationFailure , _ => CertOtherError , } }
};
}
