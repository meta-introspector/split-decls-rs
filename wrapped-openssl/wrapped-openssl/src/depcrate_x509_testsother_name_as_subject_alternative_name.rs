// Generated macro for other_name_as_subject_alternative_name (function)
macro_rules! Depcrate_x509_testsother_name_as_subject_alternative_name {
() => {
// Module: crate::x509::tests
// Provides: {"other_name_as_subject_alternative_name"}
// Dependencies: {}
# [cfg (ossl110)] # [test] fn other_name_as_subject_alternative_name () { let oid = Asn1Object :: from_str ("1.3.6.1.5.5.7.8.11") . unwrap () ; let content = [0x16 , 0x04 , 0x74 , 0x65 , 0x73 , 0x74] ; let mut builder = X509Builder :: new () . unwrap () ; let san = SubjectAlternativeName :: new () . other_name2 (oid , & content) . build (& builder . x509v3_context (None , None)) . unwrap () ; builder . append_extension (san) . unwrap () ; let cert = builder . build () ; let general_name = cert . subject_alt_names () . into_iter () . flatten () . next () . unwrap () ; unsafe { assert_eq ! ((* general_name . as_ptr ()) . type_ , 0) ; } }
};
}
