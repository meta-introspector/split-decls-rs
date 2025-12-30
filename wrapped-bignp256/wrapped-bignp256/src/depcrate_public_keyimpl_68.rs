// Generated macro for impl_68 (impl)
macro_rules! Depcrate_public_keyimpl_68 {
() => {
// Module: crate::public_key
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl EncodePublicKey for PublicKey { fn to_public_key_der (& self) -> pkcs8 :: spki :: Result < der :: Document > { let pk_bytes = self . to_bytes () ; let subject_public_key = der :: asn1 :: BitStringRef :: new (0 , & pk_bytes) ? ; pkcs8 :: SubjectPublicKeyInfo { algorithm : Self :: ALGORITHM_IDENTIFIER , subject_public_key , } . try_into () } }
};
}
