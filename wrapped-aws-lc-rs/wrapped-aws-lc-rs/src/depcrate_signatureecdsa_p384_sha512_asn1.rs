// Generated macro for ECDSA_P384_SHA512_ASN1 (const)
macro_rules! Depcrate_signatureECDSA_P384_SHA512_ASN1 {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P384_SHA512_ASN1"}
// Dependencies: {}
# [doc = " *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-512."] pub const ECDSA_P384_SHA512_ASN1 : EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm { id : & ec :: signature :: AlgorithmID :: ECDSA_P384 , digest : & digest :: SHA512 , sig_format : EcdsaSignatureFormat :: ASN1 , } ;
};
}
