// Generated macro for encode_basic_auth (function)
macro_rules! Depcrate_client_proxy_matcherencode_basic_auth {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"encode_basic_auth"}
// Dependencies: {}
fn encode_basic_auth (user : & str , pass : Option < & str >) -> HeaderValue { use base64 :: prelude :: BASE64_STANDARD ; use base64 :: write :: EncoderWriter ; use std :: io :: Write ; let mut buf = b"Basic " . to_vec () ; { let mut encoder = EncoderWriter :: new (& mut buf , & BASE64_STANDARD) ; let _ = write ! (encoder , "{user}:") ; if let Some (password) = pass { let _ = write ! (encoder , "{password}") ; } } let mut header = HeaderValue :: from_bytes (& buf) . expect ("base64 is always valid HeaderValue") ; header . set_sensitive (true) ; header }
};
}
