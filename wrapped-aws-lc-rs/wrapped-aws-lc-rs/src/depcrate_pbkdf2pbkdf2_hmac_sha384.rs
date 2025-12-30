// Generated macro for PBKDF2_HMAC_SHA384 (const)
macro_rules! Depcrate_pbkdf2PBKDF2_HMAC_SHA384 {
() => {
// Module: crate::pbkdf2
// Provides: {"PBKDF2_HMAC_SHA384"}
// Dependencies: {}
# [doc = " PBKDF2 using HMAC-SHA384."] pub const PBKDF2_HMAC_SHA384 : Algorithm = Algorithm { algorithm : hmac :: HMAC_SHA384 , max_output_len : MAX_USIZE32 * digest :: SHA384_OUTPUT_LEN as u64 , } ;
};
}
