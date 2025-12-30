// Generated macro for ClientCertVerifierBuilder (struct)
macro_rules! Depcrate_verifierClientCertVerifierBuilder {
() => {
// Module: crate::verifier
// Provides: {"ClientCertVerifierBuilder"}
// Dependencies: {}
pub (crate) struct ClientCertVerifierBuilder { provider : Option < Arc < CryptoProvider > > , roots : Arc < RootCertStore > , root_hint_subjects : Vec < DistinguishedName > , crls : Vec < CertificateRevocationListDer < 'static > > , revocation_depth : RevocationCheckDepth , revocation_policy : UnknownStatusPolicy , allow_unauthenticated : bool , }
};
}
