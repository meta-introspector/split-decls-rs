// Generated macro for CrlStatus (enum)
macro_rules! Depcrate_x509CrlStatus {
() => {
// Module: crate::x509
// Provides: {"CrlStatus"}
// Dependencies: {}
# [doc = " The status of a certificate in a revoction list"] # [doc = ""] # [doc = " Corresponds to the return value from the [`X509_CRL_get0_by_*`] methods."] # [doc = ""] # [doc = " [`X509_CRL_get0_by_*`]: https://docs.openssl.org/master/man3/X509_CRL_get0_by_serial/"] pub enum CrlStatus < 'a > { # [doc = " The certificate is not present in the list"] NotRevoked , # [doc = " The certificate is in the list and is revoked"] Revoked (& 'a X509RevokedRef) , # [doc = " The certificate is in the list, but has the \"removeFromCrl\" status."] # [doc = ""] # [doc = " This can occur if the certificate was revoked with the \"CertificateHold\""] # [doc = " reason, and has since been unrevoked."] RemoveFromCrl (& 'a X509RevokedRef) , }
};
}
