// Generated macro for tests (module)
macro_rules! Depcrate_eventsource_futurestests {
() => {
// Module: crate::eventsource::futures
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use futures :: StreamExt ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] async fn eventsource_works () { let sse_echo_server_url = option_env ! ("SSE_ECHO_SERVER_URL") . expect ("Did you set SSE_ECHO_SERVER_URL?") ; let mut es = EventSource :: new (sse_echo_server_url) . unwrap () ; let mut servers = es . subscribe ("server") . unwrap () ; let mut requests = es . subscribe ("request") . unwrap () ; assert_eq ! (servers . next () . await . unwrap () . unwrap () . 0 , "server") ; assert_eq ! (requests . next () . await . unwrap () . unwrap () . 0 , "request") ; } # [wasm_bindgen_test] async fn eventsource_connect_failure_works () { let mut es = EventSource :: new ("rubbish") . unwrap () ; let mut servers = es . subscribe ("server") . unwrap () ; assert_eq ! (servers . next () . await , Some (Err (EventSourceError :: ConnectionError))) ; } }
};
}
