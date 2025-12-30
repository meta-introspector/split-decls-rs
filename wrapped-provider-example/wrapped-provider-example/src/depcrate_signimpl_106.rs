// Generated macro for impl_106 (impl)
macro_rules! Depcrate_signimpl_106 {
() => {
// Module: crate::sign
// Provides: {"impl_106"}
// Dependencies: {}
impl TryFrom < PrivatePkcs8KeyDer < '_ > > for EcdsaSigningKeyP256 { type Error = pkcs8 :: Error ; fn try_from (value : PrivatePkcs8KeyDer < '_ >) -> Result < Self , Self :: Error > { Ok (Self { key : Arc :: new (p256 :: ecdsa :: SigningKey :: from_pkcs8_der (value . secret_pkcs8_der () ,) ?) , scheme : SignatureScheme :: ECDSA_NISTP256_SHA256 , }) } }
};
}
