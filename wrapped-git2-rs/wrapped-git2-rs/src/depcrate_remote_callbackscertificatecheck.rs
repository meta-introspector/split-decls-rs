// Generated macro for CertificateCheck (type)
macro_rules! Depcrate_remote_callbacksCertificateCheck {
() => {
// Module: crate::remote_callbacks
// Provides: {"CertificateCheck"}
// Dependencies: {}
# [doc = " Callback for a custom certificate check."] # [doc = ""] # [doc = " The first argument is the certificate received on the connection."] # [doc = " Certificates are typically either an SSH or X509 certificate."] # [doc = ""] # [doc = " The second argument is the hostname for the connection is passed as the last"] # [doc = " argument."] pub type CertificateCheck < 'a > = dyn FnMut (& Cert < '_ > , & str) -> Result < CertificateCheckStatus , Error > + 'a ;
};
}
