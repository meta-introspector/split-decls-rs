// Generated macro for test_alpn_server_select_none (function)
macro_rules! Depcrate_ssl_testtest_alpn_server_select_none {
() => {
// Module: crate::ssl::test
// Provides: {"test_alpn_server_select_none"}
// Dependencies: {}
# [test] fn test_alpn_server_select_none () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_alpn_select_callback (| _ , client | { CALLED_BACK . store (true , Ordering :: SeqCst) ; ssl :: select_next_proto (b"\x08http/1.1\x08spdy/3.1" , client) . ok_or (ssl :: AlpnError :: NOACK) }) ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_alpn_protos (b"\x06http/2") . unwrap () ; let s = client . connect () ; assert_eq ! (None , s . ssl () . selected_alpn_protocol ()) ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
