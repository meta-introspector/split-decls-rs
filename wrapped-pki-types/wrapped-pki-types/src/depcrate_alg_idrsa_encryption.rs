// Generated macro for RSA_ENCRYPTION (const)
macro_rules! Depcrate_alg_idRSA_ENCRYPTION {
() => {
// Module: crate::alg_id
// Provides: {"RSA_ENCRYPTION"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `rsaEncryption`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # rsaEncryption"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.113549.1.1.1 }"] # [doc = " NULL {}"] # [doc = " ```"] pub const RSA_ENCRYPTION : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-encryption.der")) ;
};
}
