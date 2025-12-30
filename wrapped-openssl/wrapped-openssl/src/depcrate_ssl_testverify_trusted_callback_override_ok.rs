// Generated macro for verify_trusted_callback_override_ok (function)
macro_rules! Depcrate_ssl_testverify_trusted_callback_override_ok {
() => {
// Module: crate::ssl::test
// Provides: {"verify_trusted_callback_override_ok"}
// Dependencies: {}
# [test] fn verify_trusted_callback_override_ok () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let server = Server :: builder () . build () ; let mut client = server . client () ; client . ctx () . set_ca_file ("test/root-ca.pem") . unwrap () ; client . ctx () . set_verify_callback (SslVerifyMode :: PEER , | _ , x509 | { CALLED_BACK . store (true , Ordering :: SeqCst) ; assert ! (x509 . current_cert () . is_some ()) ; true }) ; client . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
