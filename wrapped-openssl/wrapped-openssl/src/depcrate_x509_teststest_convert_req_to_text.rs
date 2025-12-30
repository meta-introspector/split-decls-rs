// Generated macro for test_convert_req_to_text (function)
macro_rules! Depcrate_x509_teststest_convert_req_to_text {
() => {
// Module: crate::x509::tests
// Provides: {"test_convert_req_to_text"}
// Dependencies: {}
# [test] fn test_convert_req_to_text () { let csr = include_bytes ! ("../../test/csr.pem") ; let csr = X509Req :: from_pem (csr) . unwrap () ; const SUBSTRINGS : & [& str] = & ["Certificate Request:\n" , "Version:" , "Subject: C=AU, ST=Some-State, O=Internet Widgits Pty Ltd, CN=foobar.com\n" , "Subject Public Key Info:" , "Signature Algorithm:" ,] ; let text = String :: from_utf8 (csr . to_text () . unwrap ()) . unwrap () ; for substring in SUBSTRINGS { assert ! (text . contains (substring) , "{:?} not found inside {}" , substring , text) ; } }
};
}
