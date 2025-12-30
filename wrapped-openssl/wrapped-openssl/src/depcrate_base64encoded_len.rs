// Generated macro for encoded_len (function)
macro_rules! Depcrate_base64encoded_len {
() => {
// Module: crate::base64
// Provides: {"encoded_len"}
// Dependencies: {}
fn encoded_len (src_len : LenType) -> Option < LenType > { let mut len = (src_len / 3) . checked_mul (4) ? ; if src_len % 3 != 0 { len = len . checked_add (4) ? ; } len = len . checked_add (1) ? ; Some (len) }
};
}
