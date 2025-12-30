// Generated macro for new_session_callback (function)
macro_rules! Depcrate_ssl_testnew_session_callback {
() => {
// Module: crate::ssl::test
// Provides: {"new_session_callback"}
// Dependencies: {}
# [doc = " LibreSSL 3.2.1 enabled TLSv1.3 by default for clients and sessions do"] # [doc = " not work due to lack of PSK support. The test passes with NO_TLSV1_3,"] # [doc = " but let's ignore it until LibreSSL supports it out of the box."] # [test] # [cfg_attr (libressl , ignore)] fn new_session_callback () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_session_id_context (b"foo") . unwrap () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_session_cache_mode (SslSessionCacheMode :: CLIENT | SslSessionCacheMode :: NO_INTERNAL) ; client . ctx () . set_new_session_callback (| _ , _ | CALLED_BACK . store (true , Ordering :: SeqCst)) ; client . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
