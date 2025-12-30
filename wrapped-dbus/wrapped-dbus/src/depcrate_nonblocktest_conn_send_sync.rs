// Generated macro for test_conn_send_sync (function)
macro_rules! Depcrate_nonblocktest_conn_send_sync {
() => {
// Module: crate::nonblock
// Provides: {"test_conn_send_sync"}
// Dependencies: {}
# [test] fn test_conn_send_sync () { fn is_send < T : Send > () { } fn is_sync < T : Sync > () { } is_send :: < Connection > () ; is_send :: < SyncConnection > () ; is_sync :: < SyncConnection > () ; is_send :: < MsgMatch > () ; }
};
}
