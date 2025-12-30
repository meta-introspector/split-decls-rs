// Generated macro for RSA_PSS_2048_8192_SHA256 (const)
macro_rules! Depcrate_signatureRSA_PSS_2048_8192_SHA256 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_2048_8192_SHA256"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-256."] pub const RSA_PSS_2048_8192_SHA256 : RsaParameters = RsaParameters :: new (& digest :: SHA256 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , 2048 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PSS_2048_8192_SHA256 ,) ;
};
}
