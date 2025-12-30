// Generated macro for ECDSA_P521_SHA256_ASN1_SIGNING (const)
macro_rules! Depcrate_signatureECDSA_P521_SHA256_ASN1_SIGNING {
() => {
// Module: crate::signature
// Provides: {"ECDSA_P521_SHA256_ASN1_SIGNING"}
// Dependencies: {}
# [doc = " Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-256."] # [doc = " # ⚠\u{fe0f} Warning"] # [doc = " The security design strength of SHA-256 digests is less then security strength of P-521."] # [doc = " This scheme should only be used for backwards compatibility purposes."] pub const ECDSA_P521_SHA256_ASN1_SIGNING : EcdsaSigningAlgorithm = EcdsaSigningAlgorithm (& ECDSA_P521_SHA256_ASN1) ;
};
}
