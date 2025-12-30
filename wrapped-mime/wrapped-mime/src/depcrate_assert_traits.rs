// Generated macro for _assert_traits (function)
macro_rules! Depcrate_assert_traits {
() => {
// Module: crate
// Provides: {"_assert_traits"}
// Dependencies: {}
fn _assert_traits () { fn assert_send_sync < T : Send + Sync > () { } assert_send_sync :: < InvalidMime > () ; assert_send_sync :: < MediaRange > () ; assert_send_sync :: < MediaType > () ; assert_send_sync :: < Value > () ; }
};
}
