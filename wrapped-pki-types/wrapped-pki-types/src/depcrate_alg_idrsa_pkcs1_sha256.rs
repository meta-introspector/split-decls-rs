// Generated macro for RSA_PKCS1_SHA256 (const)
macro_rules! Depcrate_alg_idRSA_PKCS1_SHA256 {
() => {
// Module: crate::alg_id
// Provides: {"RSA_PKCS1_SHA256"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `sha256WithRSAEncryption`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # sha256WithRSAEncryption"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.113549.1.1.11 }"] # [doc = " NULL {}"] # [doc = " ```"] pub const RSA_PKCS1_SHA256 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-pkcs1-sha256.der")) ;
};
}
