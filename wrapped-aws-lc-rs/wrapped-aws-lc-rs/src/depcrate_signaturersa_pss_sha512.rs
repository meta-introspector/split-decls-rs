// Generated macro for RSA_PSS_SHA512 (const)
macro_rules! Depcrate_signatureRSA_PSS_SHA512 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_SHA512"}
// Dependencies: {}
# [doc = " RSA PSS padding using SHA-512 for RSA signatures."] pub const RSA_PSS_SHA512 : RsaSignatureEncoding = RsaSignatureEncoding :: new (& digest :: SHA512 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , & RsaSigningAlgorithmId :: RSA_PSS_SHA512 ,) ;
};
}
