// Generated macro for cert_result_to_error (function)
macro_rules! Depcrate_errorcert_result_to_error {
() => {
// Module: crate::error
// Provides: {"cert_result_to_error"}
// Dependencies: {}
# [doc = " For cert-related rustls_results, turn them into a rustls::Error."] # [doc = ""] # [doc = " For other inputs, including Ok, return rustls::Error::General."] pub (crate) fn cert_result_to_error (result : rustls_result) -> Error { use rustls :: Error :: * ; use rustls :: OtherError ; use rustls_result :: * ; match result { CertEncodingBad => InvalidCertificate (CertificateError :: BadEncoding) , CertExpired => InvalidCertificate (CertificateError :: Expired) , CertNotYetValid => InvalidCertificate (CertificateError :: NotValidYet) , CertRevoked => InvalidCertificate (CertificateError :: Revoked) , CertUnhandledCriticalExtension => { InvalidCertificate (CertificateError :: UnhandledCriticalExtension) } CertUnknownIssuer => InvalidCertificate (CertificateError :: UnknownIssuer) , CertBadSignature => InvalidCertificate (CertificateError :: BadSignature) , CertNotValidForName => InvalidCertificate (CertificateError :: NotValidForName) , CertInvalidPurpose => InvalidCertificate (CertificateError :: InvalidPurpose) , CertApplicationVerificationFailure => { InvalidCertificate (CertificateError :: ApplicationVerificationFailure) } CertExpiredRevocationList => InvalidCertificate (CertificateError :: ExpiredRevocationList) , CertOtherError => InvalidCertificate (CertificateError :: Other (OtherError (Arc :: from (Box :: from ("") ,)))) , _ => Error :: General ("" . into ()) , } }
};
}
