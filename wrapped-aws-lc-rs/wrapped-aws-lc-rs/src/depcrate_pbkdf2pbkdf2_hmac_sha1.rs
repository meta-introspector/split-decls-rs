// Generated macro for PBKDF2_HMAC_SHA1 (const)
macro_rules! Depcrate_pbkdf2PBKDF2_HMAC_SHA1 {
() => {
// Module: crate::pbkdf2
// Provides: {"PBKDF2_HMAC_SHA1"}
// Dependencies: {}
# [doc = " PBKDF2 using HMAC-SHA1."] pub const PBKDF2_HMAC_SHA1 : Algorithm = Algorithm { algorithm : hmac :: HMAC_SHA1_FOR_LEGACY_USE_ONLY , max_output_len : MAX_USIZE32 * digest :: SHA1_OUTPUT_LEN as u64 , } ;
};
}
