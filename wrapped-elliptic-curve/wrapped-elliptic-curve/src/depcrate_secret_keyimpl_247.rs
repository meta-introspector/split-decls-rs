// Generated macro for impl_247 (impl)
macro_rules! Depcrate_secret_keyimpl_247 {
() => {
// Module: crate::secret_key
// Provides: {"impl_247"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > sec1 :: DecodeEcPrivateKey for SecretKey < C > where C : AssociatedOid + Curve + ValidatePublicKey , FieldBytesSize < C > : ModulusSize , { fn from_sec1_der (bytes : & [u8]) -> sec1 :: Result < Self > { Ok (sec1 :: EcPrivateKey :: from_der (bytes) ? . try_into () ?) } }
};
}
