// Generated macro for tests (module)
macro_rules! Depcrate_common_sec_websocket_versiontests {
() => {
// Module: crate::common::sec_websocket_version
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: SecWebsocketVersion ; # [test] fn decode_v13 () { assert_eq ! (test_decode ::< SecWebsocketVersion > (& ["13"]) , Some (SecWebsocketVersion :: V13) ,) ; } # [test] fn decode_fail () { assert_eq ! (test_decode ::< SecWebsocketVersion > (& ["1"]) , None ,) ; } # [test] fn encode_v13 () { let headers = test_encode (SecWebsocketVersion :: V13) ; assert_eq ! (headers ["sec-websocket-version"] , "13") ; } }
};
}
