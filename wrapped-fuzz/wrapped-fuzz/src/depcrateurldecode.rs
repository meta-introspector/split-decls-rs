// Generated macro for urldecode (function)
macro_rules! Depcrateurldecode {
() => {
// Module: crate
// Provides: {"urldecode"}
// Dependencies: {}
fn urldecode (data : & str) -> String { let decoded = urlencoding :: decode_binary (data . as_bytes ()) ; urlencoding :: encode_binary (& decoded [..]) . to_string () }
};
}
