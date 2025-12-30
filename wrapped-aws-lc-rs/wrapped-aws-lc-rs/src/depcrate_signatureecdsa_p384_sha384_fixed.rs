// Generated macro for ECDSA_P384_SHA384_FIXED (const)
macro_rules! Depcrate_signatureECDSA_P384_SHA384_FIXED {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P384_SHA384_FIXED"}
// Dependencies: {}
# [doc = " Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-384 curve and SHA-384."] pub const ECDSA_P384_SHA384_FIXED : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P384 , digest : & digest :: SHA384 , sig_format : EcdsaSignatureFormat :: Fixed , } ;
};
}
