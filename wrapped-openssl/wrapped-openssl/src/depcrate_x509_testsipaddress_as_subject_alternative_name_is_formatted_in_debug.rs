// Generated macro for ipaddress_as_subject_alternative_name_is_formatted_in_debug (function)
macro_rules! Depcrate_x509_testsipaddress_as_subject_alternative_name_is_formatted_in_debug {
() => {
// Module: crate::x509::tests
// Provides: {"ipaddress_as_subject_alternative_name_is_formatted_in_debug"}
// Dependencies: {}
# [cfg (ossl110)] fn ipaddress_as_subject_alternative_name_is_formatted_in_debug < T > (expected_ip : T) where T : Into < std :: net :: IpAddr > , { let expected_ip = format ! ("{:?}" , expected_ip . into ()) ; let mut builder = X509Builder :: new () . unwrap () ; let san = SubjectAlternativeName :: new () . ip (& expected_ip) . build (& builder . x509v3_context (None , None)) . unwrap () ; builder . append_extension (san) . unwrap () ; let cert = builder . build () ; let actual_ip = cert . subject_alt_names () . into_iter () . flatten () . map (| n | format ! ("{:?}" , * n)) . next () . unwrap () ; assert_eq ! (actual_ip , expected_ip) ; }
};
}
