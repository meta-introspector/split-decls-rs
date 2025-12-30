// Generated macro for RSA_PSS_SHA384 (const)
macro_rules! Depcrate_signatureRSA_PSS_SHA384 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_SHA384"}
// Dependencies: {}
# [doc = " RSA PSS padding using SHA-384 for RSA signatures."] pub const RSA_PSS_SHA384 : RsaSignatureEncoding = RsaSignatureEncoding :: new (& digest :: SHA384 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , & RsaSigningAlgorithmId :: RSA_PSS_SHA384 ,) ;
};
}
