// Generated macro for tests (module)
macro_rules! Depcrate_websocket_io_utiltests {
() => {
// Module: crate::websocket::io_util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use futures :: { AsyncReadExt , AsyncWriteExt , StreamExt } ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] async fn check_read_write () { let ws_echo_server_url = option_env ! ("WS_ECHO_SERVER_URL") . expect ("Did you set WS_ECHO_SERVER_URL?") ; let mut ws = WebSocket :: open (ws_echo_server_url) . unwrap () ; let _ = ws . next () . await . unwrap () ; let (mut reader , mut writer) = AsyncReadExt :: split (ws) ; writer . write_all (b"test 1") . await . unwrap () ; writer . write_all (b"test 2") . await . unwrap () ; let mut buf = [0u8 ; 6] ; reader . read_exact (& mut buf) . await . unwrap () ; assert_eq ! (& buf , b"test 1") ; reader . read_exact (& mut buf) . await . unwrap () ; assert_eq ! (& buf , b"test 2") ; } # [wasm_bindgen_test] async fn with_pending_bytes () { let ws_echo_server_url = option_env ! ("WS_ECHO_SERVER_URL") . expect ("Did you set WS_ECHO_SERVER_URL?") ; let mut ws = WebSocket :: open (ws_echo_server_url) . unwrap () ; let _ = ws . next () . await . unwrap () ; ws . write_all (b"1234567890") . await . unwrap () ; let mut buf = [0u8 ; 5] ; ws . read_exact (& mut buf) . await . unwrap () ; assert_eq ! (& buf , b"12345") ; assert ! (ws . has_pending_bytes ()) ; ws . read_exact (& mut buf) . await . unwrap () ; assert_eq ! (& buf , b"67890") ; assert ! (! ws . has_pending_bytes ()) ; } }
};
}
