// Generated macro for peer_certificate (function)
macro_rules! Depcrate_ssl_testpeer_certificate {
() => {
// Module: crate::ssl::test
// Provides: {"peer_certificate"}
// Dependencies: {}
# [test] fn peer_certificate () { let server = Server :: builder () . build () ; let s = server . client () . connect () ; let cert = s . ssl () . peer_certificate () . unwrap () ; let fingerprint = cert . digest (MessageDigest :: sha1 ()) . unwrap () ; assert_eq ! (hex :: encode (fingerprint) , "59172d9313e84459bcff27f967e79e6e9217e584") ; }
};
}
