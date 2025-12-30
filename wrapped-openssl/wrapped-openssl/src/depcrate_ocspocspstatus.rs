// Generated macro for OcspStatus (struct)
macro_rules! Depcrate_ocspOcspStatus {
() => {
// Module: crate::ocsp
// Provides: {"OcspStatus"}
// Dependencies: {}
pub struct OcspStatus < 'a > { # [doc = " The overall status of the response."] pub status : OcspCertStatus , # [doc = " If `status` is `CERT_STATUS_REVOKED`, the reason for the revocation."] pub reason : OcspRevokedStatus , # [doc = " If `status` is `CERT_STATUS_REVOKED`, the time at which the certificate was revoked."] pub revocation_time : Option < & 'a Asn1GeneralizedTimeRef > , # [doc = " The time that this revocation check was performed."] pub this_update : & 'a Asn1GeneralizedTimeRef , # [doc = " The time at which this revocation check expires."] # [doc = ""] # [doc = " # Deprecated"] # [doc = " Contains a sentinel maximum time (99991231235959Z) when the field is"] # [doc = " not present in the response."] # [doc = " Use [`next_update()`](Self::next_update) instead."] # [deprecated (since = "0.10.75" , note = "Use the next_update() method instead")] pub next_update : & 'a Asn1GeneralizedTimeRef , next_update_opt : Option < & 'a Asn1GeneralizedTimeRef > , }
};
}
