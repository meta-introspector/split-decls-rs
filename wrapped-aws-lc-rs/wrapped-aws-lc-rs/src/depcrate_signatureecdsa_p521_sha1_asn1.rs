// Generated macro for ECDSA_P521_SHA1_ASN1 (const)
macro_rules! Depcrate_signatureECDSA_P521_SHA1_ASN1 {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P521_SHA1_ASN1"}
// Dependencies: {}
# [doc = " Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-1."] pub const ECDSA_P521_SHA1_ASN1 : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P521 , digest : & digest :: SHA1_FOR_LEGACY_USE_ONLY , sig_format : EcdsaSignatureFormat :: ASN1 , } ;
};
}
