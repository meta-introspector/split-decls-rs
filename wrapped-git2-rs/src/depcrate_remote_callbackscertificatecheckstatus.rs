// Generated macro for CertificateCheckStatus (enum)
macro_rules! Depcrate_remote_callbacksCertificateCheckStatus {
() => {
// Module: crate::remote_callbacks
// Provides: {"CertificateCheckStatus"}
// Dependencies: {}
# [doc = " The return value for the [`RemoteCallbacks::certificate_check`] callback."] pub enum CertificateCheckStatus { # [doc = " Indicates that the certificate should be accepted."] CertificateOk , # [doc = " Indicates that the certificate callback is neither accepting nor"] # [doc = " rejecting the certificate. The result of the certificate checks"] # [doc = " built-in to libgit2 will be used instead."] CertificatePassthrough , }
};
}
