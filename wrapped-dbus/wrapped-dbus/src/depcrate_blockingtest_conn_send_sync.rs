// Generated macro for test_conn_send_sync (function)
macro_rules! Depcrate_blockingtest_conn_send_sync {
() => {
// Module: crate::blocking
// Provides: {"test_conn_send_sync"}
// Dependencies: {}
# [test] fn test_conn_send_sync () { fn is_send < T : Send > (_ : & T) { } fn is_sync < T : Sync > (_ : & T) { } let c = SyncConnection :: new_session () . unwrap () ; is_send (& c) ; is_sync (& c) ; let c = Connection :: new_session () . unwrap () ; is_send (& c) ; }
};
}
