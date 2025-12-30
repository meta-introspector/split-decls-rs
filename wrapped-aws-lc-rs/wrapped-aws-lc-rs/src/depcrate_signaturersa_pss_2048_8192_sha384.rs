// Generated macro for RSA_PSS_2048_8192_SHA384 (const)
macro_rules! Depcrate_signatureRSA_PSS_2048_8192_SHA384 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_2048_8192_SHA384"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-384."] pub const RSA_PSS_2048_8192_SHA384 : RsaParameters = RsaParameters :: new (& digest :: SHA384 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , 2048 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PSS_2048_8192_SHA384 ,) ;
};
}
