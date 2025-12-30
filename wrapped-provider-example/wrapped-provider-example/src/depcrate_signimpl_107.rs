// Generated macro for impl_107 (impl)
macro_rules! Depcrate_signimpl_107 {
() => {
// Module: crate::sign
// Provides: {"impl_107"}
// Dependencies: {}
impl SigningKey for EcdsaSigningKeyP256 { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { Some (Box :: new (self . clone ())) } else { None } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { Some (SubjectPublicKeyInfoDer :: from (self . key . verifying_key () . to_public_key_der () . ok () ? . into_vec () ,)) } }
};
}
