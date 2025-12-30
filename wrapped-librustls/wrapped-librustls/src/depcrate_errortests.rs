// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_rustls_error () { let mut buf = [0 as c_char ; 512] ; let mut n = 0 ; rustls_result :: rustls_error (0 , & mut buf as * mut _ , buf . len () , & mut n) ; let output = String :: from_utf8 (buf [0 .. n] . iter () . map (| b | * b as u8) . collect ()) . unwrap () ; assert_eq ! (& output , "a parameter had an invalid value") ; rustls_result :: rustls_error (7000 , & mut buf as * mut _ , buf . len () , & mut n) ; let output = String :: from_utf8 (buf [0 .. n] . iter () . map (| b | * b as u8) . collect ()) . unwrap () ; assert_eq ! (& output , "OK") ; rustls_result :: rustls_error (7101 , & mut buf as * mut _ , buf . len () , & mut n) ; let output = String :: from_utf8 (buf [0 .. n] . iter () . map (| b | * b as u8) . collect ()) . unwrap () ; assert_eq ! (& output , "peer sent no certificates") ; } # [test] fn test_rustls_error_into_empty_buffer () { let mut n = 99 ; rustls_result :: rustls_error (0 , & mut [] as * mut _ , 0 , & mut n) ; assert_eq ! (n , 0) ; } # [test] fn test_rustls_result_is_cert_error () { assert ! (! rustls_result :: rustls_result_is_cert_error (0)) ; assert ! (! rustls_result :: rustls_result_is_cert_error (7000)) ; for id in 7121 ..= 7131 { assert ! (rustls_result :: rustls_result_is_cert_error (id)) ; } } }
};
}
