// Generated macro for decode_unix_secs (function)
macro_rules! Depcrate_tokendecode_unix_secs {
() => {
// Module: crate::token
// Provides: {"decode_unix_secs"}
// Dependencies: {}
fn decode_unix_secs < B : Buf > (buf : & mut B) -> Option < SystemTime > { Some (UNIX_EPOCH + Duration :: from_secs (buf . get :: < u64 > () . ok () ?)) }
};
}
