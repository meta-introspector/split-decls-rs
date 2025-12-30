// Generated macro for test_alpn_server_select_none_fatal (function)
macro_rules! Depcrate_ssl_testtest_alpn_server_select_none_fatal {
() => {
// Module: crate::ssl::test
// Provides: {"test_alpn_server_select_none_fatal"}
// Dependencies: {}
# [test] # [cfg (any (ossl110 , boringssl , awslc))] fn test_alpn_server_select_none_fatal () { let mut server = Server :: builder () ; server . ctx () . set_alpn_select_callback (| _ , client | { ssl :: select_next_proto (b"\x08http/1.1\x08spdy/3.1" , client) . ok_or (ssl :: AlpnError :: ALERT_FATAL) }) ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_alpn_protos (b"\x06http/2") . unwrap () ; client . connect_err () ; }
};
}
