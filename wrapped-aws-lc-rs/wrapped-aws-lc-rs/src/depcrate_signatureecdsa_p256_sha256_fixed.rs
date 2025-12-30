// Generated macro for ECDSA_P256_SHA256_FIXED (const)
macro_rules! Depcrate_signatureECDSA_P256_SHA256_FIXED {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P256_SHA256_FIXED"}
// Dependencies: {}
# [doc = " Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-256 curve and SHA-256."] pub const ECDSA_P256_SHA256_FIXED : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P256 , digest : & digest :: SHA256 , sig_format : EcdsaSignatureFormat :: Fixed , } ;
};
}
