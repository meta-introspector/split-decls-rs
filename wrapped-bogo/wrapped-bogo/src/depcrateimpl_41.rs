// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl ServerVerifier for DummyServerAuth { fn verify_identity (& self , _identity : & ServerIdentity < '_ >) -> Result < PeerVerified , Error > { if let OcspValidation :: Reject = self . ocsp { return Err (CertificateError :: InvalidOcspResponse . into ()) ; } Ok (PeerVerified :: assertion ()) } fn verify_tls12_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { self . parent . verify_tls12_signature (input) } fn verify_tls13_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { self . parent . verify_tls13_signature (input) } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { self . parent . supported_verify_schemes () } fn request_ocsp_response (& self) -> bool { true } }
};
}
