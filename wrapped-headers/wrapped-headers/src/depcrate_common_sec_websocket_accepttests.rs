// Generated macro for tests (module)
macro_rules! Depcrate_common_sec_websocket_accepttests {
() => {
// Module: crate::common::sec_websocket_accept
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn key_to_accept () { let key = test_decode :: < SecWebsocketKey > (& ["dGhlIHNhbXBsZSBub25jZQ=="]) . expect ("key") ; let accept = SecWebsocketAccept :: from (key) ; let headers = test_encode (accept) ; assert_eq ! (headers ["sec-websocket-accept"] , "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=") ; } }
};
}
