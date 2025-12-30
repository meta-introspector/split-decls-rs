// Generated macro for send_and_sync (function)
macro_rules! Depcratesend_and_sync {
() => {
// Module: crate
// Provides: {"send_and_sync"}
// Dependencies: {}
# [test] fn send_and_sync () { fn assert_send_and_sync < T : Send + Sync > () { } # [cfg (all (feature = "std" , not (target_family = "wasm")))] { assert_send_and_sync :: < Blocking > () ; assert_send_and_sync :: < Ready > () ; } assert_send_and_sync :: < NonBlocking < 'static > > () ; assert_send_and_sync :: < FutureWrapper < () > > () ; }
};
}
