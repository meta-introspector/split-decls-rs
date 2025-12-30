// Generated macro for impl_248 (impl)
macro_rules! Depcrate_secret_keyimpl_248 {
() => {
// Module: crate::secret_key
// Provides: {"impl_248"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "arithmetic" , feature = "sec1"))] impl < C > sec1 :: EncodeEcPrivateKey for SecretKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn to_sec1_der (& self) -> sec1 :: Result < der :: SecretDocument > { let private_key_bytes = Zeroizing :: new (self . to_bytes ()) ; let public_key_bytes = self . public_key () . to_encoded_point (false) ; Ok (der :: SecretDocument :: encode_msg (& sec1 :: EcPrivateKey { private_key : & private_key_bytes , parameters : Some (C :: OID . into ()) , public_key : Some (public_key_bytes . as_bytes ()) , }) ?) } }
};
}
