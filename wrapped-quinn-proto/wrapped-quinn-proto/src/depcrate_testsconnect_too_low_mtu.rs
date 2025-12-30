// Generated macro for connect_too_low_mtu (function)
macro_rules! Depcrate_testsconnect_too_low_mtu {
() => {
// Module: crate::tests
// Provides: {"connect_too_low_mtu"}
// Dependencies: {}
# [test] # [doc = " This is mostly a sanity check to ensure our testing code is correctly dropping packets above the"] # [doc = " pmtu"] fn connect_too_low_mtu () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . mtu = 1000 ; pair . begin_connect (client_config ()) ; pair . drive () ; pair . server . assert_no_accept () ; }
};
}
