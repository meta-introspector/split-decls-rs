// Generated macro for test_alpn_server_advertise_multiple (function)
macro_rules! Depcrate_ssl_testtest_alpn_server_advertise_multiple {
() => {
// Module: crate::ssl::test
// Provides: {"test_alpn_server_advertise_multiple"}
// Dependencies: {}
# [doc = " Tests that when the `SslStream` is created as a server stream, the protocols"] # [doc = " are correctly advertised to the client."] # [test] fn test_alpn_server_advertise_multiple () { let mut server = Server :: builder () ; server . ctx () . set_alpn_select_callback (| _ , client | { ssl :: select_next_proto (b"\x08http/1.1\x08spdy/3.1" , client) . ok_or (ssl :: AlpnError :: NOACK) }) ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_alpn_protos (b"\x08spdy/3.1") . unwrap () ; let s = client . connect () ; assert_eq ! (s . ssl () . selected_alpn_protocol () , Some (& b"spdy/3.1" [..])) ; }
};
}
