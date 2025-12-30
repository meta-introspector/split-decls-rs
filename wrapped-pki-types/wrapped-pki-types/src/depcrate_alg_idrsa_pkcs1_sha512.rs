// Generated macro for RSA_PKCS1_SHA512 (const)
macro_rules! Depcrate_alg_idRSA_PKCS1_SHA512 {
() => {
// Module: crate::alg_id
// Provides: {"RSA_PKCS1_SHA512"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `sha512WithRSAEncryption`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # sha512WithRSAEncryption"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.113549.1.1.13 }"] # [doc = " NULL {}"] # [doc = " ```"] pub const RSA_PKCS1_SHA512 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-pkcs1-sha512.der")) ;
};
}
