// Generated macro for ECDSA_SHA512 (const)
macro_rules! Depcrate_alg_idECDSA_SHA512 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_SHA512"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `ecdsa-with-SHA512`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecdsa-with-SHA512"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.4.3.4 }"] # [doc = " ```"] pub const ECDSA_SHA512 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-sha512.der")) ;
};
}
