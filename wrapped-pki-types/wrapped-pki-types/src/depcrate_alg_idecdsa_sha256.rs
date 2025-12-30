// Generated macro for ECDSA_SHA256 (const)
macro_rules! Depcrate_alg_idECDSA_SHA256 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_SHA256"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `ecdsa-with-SHA256`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecdsa-with-SHA256"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.4.3.2 }"] # [doc = " ```"] pub const ECDSA_SHA256 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-sha256.der")) ;
};
}
