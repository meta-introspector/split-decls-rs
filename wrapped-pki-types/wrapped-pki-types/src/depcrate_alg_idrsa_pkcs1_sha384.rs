// Generated macro for RSA_PKCS1_SHA384 (const)
macro_rules! Depcrate_alg_idRSA_PKCS1_SHA384 {
() => {
// Module: crate::alg_id
// Provides: {"RSA_PKCS1_SHA384"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `sha384WithRSAEncryption`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # sha384WithRSAEncryption"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.113549.1.1.12 }"] # [doc = " NULL {}"] # [doc = " ```"] pub const RSA_PKCS1_SHA384 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-pkcs1-sha384.der")) ;
};
}
