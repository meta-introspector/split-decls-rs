// Generated macro for impl_137 (impl)
macro_rules! Depcrate_pkcs8impl_137 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_137"}
// Dependencies: {}
impl < P > TryFrom < PrivateKeyInfoRef < '_ > > for SigningKey < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Error = :: pkcs8 :: Error ; fn try_from (private_key_info : :: pkcs8 :: PrivateKeyInfoRef < '_ >) -> :: pkcs8 :: Result < Self > { let keypair = KeyPair :: try_from (private_key_info) ? ; Ok (keypair . signing_key) } }
};
}
