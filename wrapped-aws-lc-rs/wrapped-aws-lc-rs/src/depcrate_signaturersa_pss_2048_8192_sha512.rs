// Generated macro for RSA_PSS_2048_8192_SHA512 (const)
macro_rules! Depcrate_signatureRSA_PSS_2048_8192_SHA512 {
() => {
// Module: crate::signature
// Provides: {"RSA_PSS_2048_8192_SHA512"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-512."] pub const RSA_PSS_2048_8192_SHA512 : RsaParameters = RsaParameters :: new (& digest :: SHA512 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PSS_PADDING , 2048 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PSS_2048_8192_SHA512 ,) ;
};
}
