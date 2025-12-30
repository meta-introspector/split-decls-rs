// Generated macro for keep_alive (function)
macro_rules! Depcrate_testskeep_alive {
() => {
// Module: crate::tests
// Provides: {"keep_alive"}
// Dependencies: {}
# [test] fn keep_alive () { let _guard = subscribe () ; const IDLE_TIMEOUT : u64 = 10 ; let server = ServerConfig { transport : Arc :: new (TransportConfig { keep_alive_interval : Some (Duration :: from_millis (IDLE_TIMEOUT / 2)) , max_idle_timeout : Some (VarInt (IDLE_TIMEOUT)) , .. TransportConfig :: default () }) , .. server_config () } ; let mut pair = Pair :: new (Default :: default () , server) ; let (client_ch , server_ch) = pair . connect () ; let end = pair . time + Duration :: from_millis (20 * IDLE_TIMEOUT) ; while pair . time < end { if ! pair . step () { if let Some (time) = min_opt (pair . client . next_wakeup () , pair . server . next_wakeup ()) { pair . time = time ; } } assert ! (! pair . client_conn_mut (client_ch) . is_closed ()) ; assert ! (! pair . server_conn_mut (server_ch) . is_closed ()) ; } }
};
}
