// Generated macro for decoded_len (function)
macro_rules! Depcrate_base64decoded_len {
() => {
// Module: crate::base64
// Provides: {"decoded_len"}
// Dependencies: {}
fn decoded_len (src_len : LenType) -> Option < LenType > { let mut len = (src_len / 4) . checked_mul (3) ? ; if src_len % 4 != 0 { len = len . checked_add (3) ? ; } Some (len) }
};
}
