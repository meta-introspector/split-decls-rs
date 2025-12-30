// Generated macro for test_convert_to_text (function)
macro_rules! Depcrate_x509_teststest_convert_to_text {
() => {
// Module: crate::x509::tests
// Provides: {"test_convert_to_text"}
// Dependencies: {}
# [test] fn test_convert_to_text () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; const SUBSTRINGS : & [& str] = & ["Certificate:\n" , "Serial Number:" , "Signature Algorithm:" , "Issuer: C=AU, ST=Some-State, O=Internet Widgits Pty Ltd\n" , "Subject: C=AU, ST=Some-State, O=Internet Widgits Pty Ltd, CN=foobar.com\n" , "Subject Public Key Info:" ,] ; let text = String :: from_utf8 (cert . to_text () . unwrap ()) . unwrap () ; for substring in SUBSTRINGS { assert ! (text . contains (substring) , "{:?} not found inside {}" , substring , text) ; } }
};
}
