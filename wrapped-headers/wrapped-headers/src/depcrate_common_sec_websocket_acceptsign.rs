// Generated macro for sign (function)
macro_rules! Depcrate_common_sec_websocket_acceptsign {
() => {
// Module: crate::common::sec_websocket_accept
// Provides: {"sign"}
// Dependencies: {}
fn sign (key : & [u8]) -> SecWebsocketAccept { let mut sha1 = Sha1 :: default () ; sha1 . update (key) ; sha1 . update (& b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11" [..]) ; let b64 = Bytes :: from (ENGINE . encode (sha1 . finalize ())) ; let val = HeaderValue :: from_maybe_shared (b64) . expect ("base64 is a valid value") ; SecWebsocketAccept (val) }
};
}
