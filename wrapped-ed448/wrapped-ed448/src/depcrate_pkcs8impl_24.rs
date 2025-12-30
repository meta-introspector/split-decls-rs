// Generated macro for impl_24 (impl)
macro_rules! Depcrate_pkcs8impl_24 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl EncodePublicKey for PublicKeyBytes { fn to_public_key_der (& self) -> spki :: Result < Document > { pkcs8 :: SubjectPublicKeyInfoRef { algorithm : ALGORITHM_ID , subject_public_key : BitStringRef :: new (0 , & self . 0) ? , } . try_into () } }
};
}
