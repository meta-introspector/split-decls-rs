// Generated macro for impl_30 (impl)
macro_rules! Depcrate_pkcs8impl_30 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl EncodePublicKey for PublicKeyBytes { fn to_public_key_der (& self) -> spki :: Result < Document > { pkcs8 :: SubjectPublicKeyInfoRef { algorithm : ALGORITHM_ID , subject_public_key : BitStringRef :: new (0 , & self . 0) ? , } . try_into () } }
};
}
