// Generated macro for certificate_from_pem (function)
macro_rules! Depcrate_testcertificate_from_pem {
() => {
// Module: crate::test
// Provides: {"certificate_from_pem"}
// Dependencies: {}
# [test] fn certificate_from_pem () { let dir = tempfile :: tempdir () . unwrap () ; let keys = test_cert_gen :: keys () ; let der_path = dir . path () . join ("cert.der") ; fs :: write (& der_path , keys . client . ca . get_der ()) . unwrap () ; let output = Command :: new ("openssl") . arg ("x509") . arg ("-in") . arg (der_path) . arg ("-inform") . arg ("der") . stderr (Stdio :: piped ()) . output () . unwrap () ; assert ! (output . status . success ()) ; let cert = Certificate :: from_pem (& output . stdout) . unwrap () ; assert_eq ! (cert . to_der () . unwrap () , keys . client . ca . get_der ()) ; }
};
}
