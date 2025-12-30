// Generated macro for test_add_name_entry (function)
macro_rules! Depcrate_x509_teststest_add_name_entry {
() => {
// Module: crate::x509::tests
// Provides: {"test_add_name_entry"}
// Dependencies: {}
# [test] fn test_add_name_entry () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let inp_name = cert . subject_name () . entries () . next () . unwrap () ; let mut names = X509Name :: builder () . unwrap () ; names . append_entry (inp_name) . unwrap () ; let names = names . build () ; let mut entries = names . entries () ; let outp_name = entries . next () . unwrap () ; assert_eq ! (outp_name . object () . nid () , inp_name . object () . nid ()) ; assert_eq ! (outp_name . data () . as_slice () , inp_name . data () . as_slice ()) ; assert ! (entries . next () . is_none ()) ; }
};
}
