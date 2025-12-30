// Generated macro for ECDSA_P521 (const)
macro_rules! Depcrate_alg_idECDSA_P521 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_P521"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecPublicKey"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }"] # [doc = " # secp521r1"] # [doc = " OBJECT_IDENTIFIER { 1.3.132.0.35 }"] # [doc = " ```"] pub const ECDSA_P521 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-p521.der")) ;
};
}
