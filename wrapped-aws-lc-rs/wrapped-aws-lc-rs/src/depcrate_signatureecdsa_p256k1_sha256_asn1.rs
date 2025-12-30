// Generated macro for ECDSA_P256K1_SHA256_ASN1 (const)
macro_rules! Depcrate_signatureECDSA_P256K1_SHA256_ASN1 {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P256K1_SHA256_ASN1"}
// Dependencies: {}
# [doc = " Verification of ASN.1 DER-encoded ECDSA signatures using the P-256K1 curve and SHA-256."] pub const ECDSA_P256K1_SHA256_ASN1 : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P256K1 , digest : & digest :: SHA256 , sig_format : EcdsaSignatureFormat :: ASN1 , } ;
};
}
