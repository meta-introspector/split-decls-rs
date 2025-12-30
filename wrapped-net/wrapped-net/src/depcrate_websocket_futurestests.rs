// Generated macro for tests (module)
macro_rules! Depcrate_websocket_futurestests {
() => {
// Module: crate::websocket::futures
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use futures :: { SinkExt , StreamExt } ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] async fn websocket_works () { let ws_echo_server_url = option_env ! ("WS_ECHO_SERVER_URL") . expect ("Did you set WS_ECHO_SERVER_URL?") ; let ws = WebSocket :: open (ws_echo_server_url) . unwrap () ; let (mut sender , mut receiver) = ws . split () ; sender . send (Message :: Text (String :: from ("test 1"))) . await . unwrap () ; sender . send (Message :: Text (String :: from ("test 2"))) . await . unwrap () ; let _ = receiver . next () . await ; assert_eq ! (receiver . next () . await . unwrap () . unwrap () , Message :: Text ("test 1" . to_string ())) ; assert_eq ! (receiver . next () . await . unwrap () . unwrap () , Message :: Text ("test 2" . to_string ())) ; } }
};
}
