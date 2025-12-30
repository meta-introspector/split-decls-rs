// Generated macro for ECDSA_P384_SHA3_384_ASN1 (const)
macro_rules! Depcrate_signatureECDSA_P384_SHA3_384_ASN1 {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P384_SHA3_384_ASN1"}
// Dependencies: {}
# [doc = " Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA3-384."] pub const ECDSA_P384_SHA3_384_ASN1 : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P384 , digest : & digest :: SHA3_384 , sig_format : EcdsaSignatureFormat :: ASN1 , } ;
};
}
