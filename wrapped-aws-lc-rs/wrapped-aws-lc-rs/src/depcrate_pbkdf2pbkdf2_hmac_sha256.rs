// Generated macro for PBKDF2_HMAC_SHA256 (const)
macro_rules! Depcrate_pbkdf2PBKDF2_HMAC_SHA256 {
() => {
// Module: crate::pbkdf2
// Provides: {"PBKDF2_HMAC_SHA256"}
// Dependencies: {}
# [doc = " PBKDF2 using HMAC-SHA256."] pub const PBKDF2_HMAC_SHA256 : Algorithm = Algorithm { algorithm : hmac :: HMAC_SHA256 , max_output_len : MAX_USIZE32 * digest :: SHA256_OUTPUT_LEN as u64 , } ;
};
}
