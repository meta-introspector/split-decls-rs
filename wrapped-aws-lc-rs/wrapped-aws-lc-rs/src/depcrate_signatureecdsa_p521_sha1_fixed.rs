// Generated macro for ECDSA_P521_SHA1_FIXED (const)
macro_rules! Depcrate_signatureECDSA_P521_SHA1_FIXED {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P521_SHA1_FIXED"}
// Dependencies: {}
# [doc = " Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-1."] pub const ECDSA_P521_SHA1_FIXED : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P521 , digest : & digest :: SHA1_FOR_LEGACY_USE_ONLY , sig_format : EcdsaSignatureFormat :: Fixed , } ;
};
}
