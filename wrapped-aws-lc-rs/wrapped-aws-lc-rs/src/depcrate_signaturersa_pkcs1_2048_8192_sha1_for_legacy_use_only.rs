// Generated macro for RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY (const)
macro_rules! Depcrate_signatureRSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY {
() => {
// Module: crate::signature
// Provides: {"RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 2048-8192 bits, PKCS#1.5 padding, and SHA-1."] pub const RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY : RsaParameters = RsaParameters :: new (& digest :: SHA1_FOR_LEGACY_USE_ONLY , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PADDING , 2048 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY ,) ;
};
}
