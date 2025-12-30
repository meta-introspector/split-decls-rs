// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl ClientVerifier for DummyClientAuth { fn verify_identity (& self , _identity : & ClientIdentity < '_ >) -> Result < PeerVerified , Error > { Ok (PeerVerified :: assertion ()) } fn verify_tls12_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { self . parent . verify_tls12_signature (input) } fn verify_tls13_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { self . parent . verify_tls13_signature (input) } fn root_hint_subjects (& self) -> Arc < [DistinguishedName] > { self . root_hint_subjects . clone () } fn client_auth_mandatory (& self) -> bool { self . mandatory } fn offer_client_auth (& self) -> bool { true } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { self . parent . supported_verify_schemes () } }
};
}
