// Generated macro for ECDSA_P521_SHA256_FIXED (const)
macro_rules! Depcrate_signatureECDSA_P521_SHA256_FIXED {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P521_SHA256_FIXED"}
// Dependencies: {}
# [doc = " Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-256."] pub const ECDSA_P521_SHA256_FIXED : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P521 , digest : & digest :: SHA256 , sig_format : EcdsaSignatureFormat :: Fixed , } ;
};
}
