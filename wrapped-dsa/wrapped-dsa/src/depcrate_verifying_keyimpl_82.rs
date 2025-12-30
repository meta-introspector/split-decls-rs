// Generated macro for impl_82 (impl)
macro_rules! Depcrate_verifying_keyimpl_82 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl EncodePublicKey for VerifyingKey { fn to_public_key_der (& self) -> spki :: Result < spki :: Document > { let parameters = self . components . to_der () ? ; let parameters = AnyRef :: from_der (& parameters) ? ; let algorithm = AlgorithmIdentifierRef { oid : OID , parameters : Some (parameters) , } ; let y_bytes = self . y . to_be_bytes () ; let y = UintRef :: new (& y_bytes) ? ; let public_key = y . to_der () ? ; SubjectPublicKeyInfoRef { algorithm , subject_public_key : BitStringRef :: new (0 , & public_key) ? , } . try_into () } }
};
}
