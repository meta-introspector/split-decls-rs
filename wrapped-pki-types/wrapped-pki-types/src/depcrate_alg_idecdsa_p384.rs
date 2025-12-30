// Generated macro for ECDSA_P384 (const)
macro_rules! Depcrate_alg_idECDSA_P384 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_P384"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecPublicKey"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }"] # [doc = " # secp384r1"] # [doc = " OBJECT_IDENTIFIER { 1.3.132.0.34 }"] # [doc = " ```"] pub const ECDSA_P384 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-p384.der")) ;
};
}
