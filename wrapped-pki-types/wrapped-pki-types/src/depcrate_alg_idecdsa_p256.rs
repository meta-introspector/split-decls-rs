// Generated macro for ECDSA_P256 (const)
macro_rules! Depcrate_alg_idECDSA_P256 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_P256"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecPublicKey"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }"] # [doc = " # secp256r1"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.3.1.7 }"] # [doc = " ```"] pub const ECDSA_P256 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-p256.der")) ;
};
}
