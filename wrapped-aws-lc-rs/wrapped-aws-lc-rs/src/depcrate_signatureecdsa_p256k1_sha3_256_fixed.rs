// Generated macro for ECDSA_P256K1_SHA3_256_FIXED (const)
macro_rules! Depcrate_signatureECDSA_P256K1_SHA3_256_FIXED {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P256K1_SHA3_256_FIXED"}
// Dependencies: {}
# [doc = " Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-256K1 curve and SHA3-256."] pub const ECDSA_P256K1_SHA3_256_FIXED : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P256K1 , digest : & digest :: SHA3_256 , sig_format : EcdsaSignatureFormat :: Fixed , } ;
};
}
