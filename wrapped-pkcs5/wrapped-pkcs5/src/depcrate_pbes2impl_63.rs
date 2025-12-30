// Generated macro for impl_63 (impl)
macro_rules! Depcrate_pbes2impl_63 {
() => {
// Module: crate::pbes2
// Provides: {"impl_63"}
// Dependencies: {}
impl EncryptionScheme { # [doc = " Get the size of a key used by this algorithm in bytes."] pub fn key_size (& self) -> usize { match self { Self :: Aes128Cbc { .. } => 16 , Self :: Aes192Cbc { .. } => 24 , Self :: Aes256Cbc { .. } => 32 , Self :: Aes128Gcm { .. } => 16 , Self :: Aes256Gcm { .. } => 32 , # [cfg (feature = "des-insecure")] Self :: DesCbc { .. } => 8 , # [cfg (feature = "3des")] Self :: DesEde3Cbc { .. } => 24 , } } # [doc = " Get the [`ObjectIdentifier`] (a.k.a OID) for this algorithm."] pub fn oid (& self) -> ObjectIdentifier { match self { Self :: Aes128Cbc { .. } => AES_128_CBC_OID , Self :: Aes192Cbc { .. } => AES_192_CBC_OID , Self :: Aes256Cbc { .. } => AES_256_CBC_OID , Self :: Aes128Gcm { .. } => AES_128_GCM_OID , Self :: Aes256Gcm { .. } => AES_256_GCM_OID , # [cfg (feature = "des-insecure")] Self :: DesCbc { .. } => DES_CBC_OID , # [cfg (feature = "3des")] Self :: DesEde3Cbc { .. } => DES_EDE3_CBC_OID , } } # [doc = " Convenience function to turn the OID (see [`oid`](Self::oid))"] # [doc = " of this [`EncryptionScheme`] into error case"] # [doc = " [`Error::AlgorithmParametersInvalid`]"] pub fn to_alg_params_invalid (& self) -> Error { Error :: AlgorithmParametersInvalid { oid : self . oid () } } }
};
}
