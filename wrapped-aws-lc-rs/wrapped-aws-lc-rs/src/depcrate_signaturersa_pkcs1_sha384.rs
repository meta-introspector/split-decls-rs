// Generated macro for RSA_PKCS1_SHA384 (const)
macro_rules! Depcrate_signatureRSA_PKCS1_SHA384 {
() => {
// Module: crate::signature
// Provides: {"RSA_PKCS1_SHA384"}
// Dependencies: {}
# [doc = " PKCS#1 1.5 padding using SHA-384 for RSA signatures."] pub const RSA_PKCS1_SHA384 : RsaSignatureEncoding = RsaSignatureEncoding :: new (& digest :: SHA384 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PADDING , & RsaSigningAlgorithmId :: RSA_PKCS1_SHA384 ,) ;
};
}
