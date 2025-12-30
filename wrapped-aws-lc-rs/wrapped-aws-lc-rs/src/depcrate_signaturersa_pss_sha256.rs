// Generated macro for RSA_PSS_SHA256 (const)
macro_rules! Depcrate_signatureRSA_PSS_SHA256 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_SHA256"}
// Dependencies: {}
# [doc = " RSA PSS padding using SHA-256 for RSA signatures."] pub const RSA_PSS_SHA256 : RsaSignatureEncoding = RsaSignatureEncoding :: new (& digest :: SHA256 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , & RsaSigningAlgorithmId :: RSA_PSS_SHA256 ,) ;
};
}
