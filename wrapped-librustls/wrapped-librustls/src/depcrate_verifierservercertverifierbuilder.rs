// Generated macro for ServerCertVerifierBuilder (struct)
macro_rules! Depcrate_verifierServerCertVerifierBuilder {
() => {
// Module: crate::verifier
// Provides: {"ServerCertVerifierBuilder"}
// Dependencies: {}
pub (crate) struct ServerCertVerifierBuilder { provider : Option < Arc < CryptoProvider > > , roots : Arc < RootCertStore > , crls : Vec < CertificateRevocationListDer < 'static > > , revocation_depth : RevocationCheckDepth , revocation_policy : UnknownStatusPolicy , revocation_expiration_policy : ExpirationPolicy , }
};
}
