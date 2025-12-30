// Generated macro for impl_294 (impl)
macro_rules! Depcrate_public_keyimpl_294 {
() => {
// Module: crate::public_key
// Provides: {"impl_294"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > EncodePublicKey for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn to_public_key_der (& self) -> pkcs8 :: spki :: Result < der :: Document > { let public_key_bytes = self . to_encoded_point (false) ; let subject_public_key = der :: asn1 :: BitStringRef :: new (0 , public_key_bytes . as_bytes ()) ? ; pkcs8 :: SubjectPublicKeyInfo { algorithm : Self :: ALGORITHM_IDENTIFIER , subject_public_key , } . try_into () } }
};
}
