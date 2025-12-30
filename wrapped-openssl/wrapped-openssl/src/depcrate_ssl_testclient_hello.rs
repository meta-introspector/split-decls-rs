// Generated macro for client_hello (function)
macro_rules! Depcrate_ssl_testclient_hello {
() => {
// Module: crate::ssl::test
// Provides: {"client_hello"}
// Dependencies: {}
# [test] # [cfg (ossl111)] fn client_hello () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_client_hello_callback (| ssl , _ | { assert ! (! ssl . client_hello_isv2 ()) ; assert_eq ! (ssl . client_hello_legacy_version () , Some (SslVersion :: TLS1_2)) ; assert ! (ssl . client_hello_random () . is_some ()) ; assert ! (ssl . client_hello_session_id () . is_some ()) ; assert ! (ssl . client_hello_ciphers () . is_some ()) ; assert ! (ssl . client_hello_compression_methods () . is_some ()) ; assert ! (ssl . bytes_to_cipher_list (ssl . client_hello_ciphers () . unwrap () , ssl . client_hello_isv2 ()) . is_ok ()) ; CALLED_BACK . store (true , Ordering :: SeqCst) ; Ok (ClientHelloResponse :: SUCCESS) }) ; let server = server . build () ; server . client () . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
