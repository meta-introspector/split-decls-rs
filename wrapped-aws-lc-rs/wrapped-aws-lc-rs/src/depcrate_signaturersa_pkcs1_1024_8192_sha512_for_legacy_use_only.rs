// Generated macro for RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY (const)
macro_rules! Depcrate_signatureRSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY {
() => {
// Module: crate::signature
// Provides: {"RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY"}
// Dependencies: {}
# [doc = " Verification of signatures using RSA keys of 1024-8192 bits, PKCS#1.5 padding, and SHA-512."] pub const RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY : RsaParameters = RsaParameters :: new (& digest :: SHA512 , & rsa :: signature :: RsaPadding :: RSA_PKCS1_PADDING , 1024 ..= 8192 , & RsaVerificationAlgorithmId :: RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY ,) ;
};
}
