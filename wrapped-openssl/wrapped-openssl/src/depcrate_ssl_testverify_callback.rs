// Generated macro for verify_callback (function)
macro_rules! Depcrate_ssl_testverify_callback {
() => {
// Module: crate::ssl::test
// Provides: {"verify_callback"}
// Dependencies: {}
# [test] fn verify_callback () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let server = Server :: builder () . build () ; let mut client = server . client () ; let expected = "59172d9313e84459bcff27f967e79e6e9217e584" ; client . ctx () . set_verify_callback (SslVerifyMode :: PEER , move | _ , x509 | { CALLED_BACK . store (true , Ordering :: SeqCst) ; let cert = x509 . current_cert () . unwrap () ; let digest = cert . digest (MessageDigest :: sha1 ()) . unwrap () ; assert_eq ! (hex :: encode (digest) , expected) ; true }) ; client . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
