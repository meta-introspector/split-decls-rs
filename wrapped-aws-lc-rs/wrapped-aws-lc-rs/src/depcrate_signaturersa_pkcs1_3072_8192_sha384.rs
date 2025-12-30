// Generated macro for RSA_PKCS1_3072_8192_SHA384 (const)
macro_rules! Depcrate_signatureRSA_PKCS1_3072_8192_SHA384 {
() => {
// Module: crate::signature
// Provides: {"RSA_PKCS1_3072_8192_SHA384"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 3072-8192 bits, PKCS#1.5 padding, and SHA-384."] pub const RSA_PKCS1_3072_8192_SHA384 : RsaParameters = RsaParameters :: new (& digest :: SHA384 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PADDING , 3072 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PKCS1_3072_8192_SHA384 ,) ;
};
}
