// Generated macro for PBKDF2_HMAC_SHA512 (const)
macro_rules! Depcrate_pbkdf2PBKDF2_HMAC_SHA512 {
() => {
// Module: crate::pbkdf2
// Provides: {"PBKDF2_HMAC_SHA512"}
// Dependencies: {}
# [doc = " PBKDF2 using HMAC-SHA512."] pub const PBKDF2_HMAC_SHA512 : Algorithm = Algorithm { algorithm : hmac :: HMAC_SHA512 , max_output_len : MAX_USIZE32 * digest :: SHA512_OUTPUT_LEN as u64 , } ;
};
}
