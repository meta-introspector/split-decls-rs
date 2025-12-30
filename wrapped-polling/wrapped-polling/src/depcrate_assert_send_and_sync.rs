// Generated macro for _assert_send_and_sync (function)
macro_rules! Depcrate_assert_send_and_sync {
() => {
// Module: crate
// Provides: {"_assert_send_and_sync"}
// Dependencies: {}
fn _assert_send_and_sync () { fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } assert_send :: < Poller > () ; assert_sync :: < Poller > () ; assert_send :: < Event > () ; assert_sync :: < Event > () ; assert_send :: < Events > () ; }
};
}
