// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl server :: ServerCredentialResolver for FixedSignatureSchemeServerCertResolver { fn resolve (& self , client_hello : & ClientHello < '_ >) -> Result < SelectedCredential , Error > { if ! client_hello . signature_schemes () . contains (& self . scheme) { return Err (Error :: PeerIncompatible (PeerIncompatible :: NoSignatureSchemesInCommon ,)) ; } self . credentials . signer (& [self . scheme]) . ok_or (Error :: PeerIncompatible (PeerIncompatible :: NoSignatureSchemesInCommon ,)) } }
};
}
