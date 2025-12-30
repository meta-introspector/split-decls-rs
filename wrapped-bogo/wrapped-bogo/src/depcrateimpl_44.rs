// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl SigningKey for FixedSignatureSchemeSigningKey { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { self . key . choose_scheme (& [self . scheme]) } else { self . key . choose_scheme (& []) } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { self . key . public_key () } }
};
}
