// Generated macro for impl_147 (impl)
macro_rules! Depcrateimpl_147 {
() => {
// Module: crate
// Provides: {"impl_147"}
// Dependencies: {}
impl KeyProvider for Provider { fn load_private_key (& self , key_der : PrivateKeyDer < 'static > ,) -> Result < Box < dyn SigningKey > , Error > { let PrivateKeyDer :: Pkcs8 (key_der) = key_der else { return Err (Error :: General ("only PKCS#8 private keys are supported" . into () ,)) ; } ; Ok (Box :: new (sign :: EcdsaSigningKeyP256 :: try_from (key_der) . map_err (other_err) ? ,)) } }
};
}
