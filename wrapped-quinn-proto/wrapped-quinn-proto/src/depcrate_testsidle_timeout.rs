// Generated macro for idle_timeout (function)
macro_rules! Depcrate_testsidle_timeout {
() => {
// Module: crate::tests
// Provides: {"idle_timeout"}
// Dependencies: {}
# [test] fn idle_timeout () { let _guard = subscribe () ; const IDLE_TIMEOUT : u64 = 100 ; let server = ServerConfig { transport : Arc :: new (TransportConfig { max_idle_timeout : Some (VarInt (IDLE_TIMEOUT)) , .. TransportConfig :: default () }) , .. server_config () } ; let mut pair = Pair :: new (Default :: default () , server) ; let (client_ch , server_ch) = pair . connect () ; pair . client_conn_mut (client_ch) . ping () ; let start = pair . time ; while ! pair . client_conn_mut (client_ch) . is_closed () || ! pair . server_conn_mut (server_ch) . is_closed () { if ! pair . step () { if let Some (t) = min_opt (pair . client . next_wakeup () , pair . server . next_wakeup ()) { pair . time = t ; } } pair . client . inbound . clear () ; } assert ! (pair . time - start < Duration :: from_millis (2 * IDLE_TIMEOUT)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: TimedOut , })) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: TimedOut , })) ; }
};
}
