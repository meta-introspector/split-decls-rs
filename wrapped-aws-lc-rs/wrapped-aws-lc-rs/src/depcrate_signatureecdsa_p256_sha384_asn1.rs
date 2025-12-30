// Generated macro for ECDSA_P256_SHA384_ASN1 (const)
macro_rules! Depcrate_signatureECDSA_P256_SHA384_ASN1 {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P256_SHA384_ASN1"}
// Dependencies: {}
# [doc = " *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-384."] pub const ECDSA_P256_SHA384_ASN1 : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P256 , digest : & digest :: SHA384 , sig_format : EcdsaSignatureFormat :: ASN1 , } ;
};
}
