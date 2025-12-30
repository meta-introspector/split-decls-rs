// Generated macro for encode_unix_secs (function)
macro_rules! Depcrate_tokenencode_unix_secs {
() => {
// Module: crate::token
// Provides: {"encode_unix_secs"}
// Dependencies: {}
fn encode_unix_secs (buf : & mut Vec < u8 > , time : SystemTime) { buf . write :: < u64 > (time . duration_since (UNIX_EPOCH) . unwrap_or_default () . as_secs () ,) ; }
};
}
