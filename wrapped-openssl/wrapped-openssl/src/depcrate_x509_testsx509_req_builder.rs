// Generated macro for x509_req_builder (function)
macro_rules! Depcrate_x509_testsx509_req_builder {
() => {
// Module: crate::x509::tests
// Provides: {"x509_req_builder"}
// Dependencies: {}
# [test] fn x509_req_builder () { let pkey = pkey () ; let mut name = X509Name :: builder () . unwrap () ; name . append_entry_by_nid (Nid :: COMMONNAME , "foobar.com") . unwrap () ; let name = name . build () ; let mut builder = X509Req :: builder () . unwrap () ; builder . set_version (0) . unwrap () ; builder . set_subject_name (& name) . unwrap () ; builder . set_pubkey (& pkey) . unwrap () ; let mut extensions = Stack :: new () . unwrap () ; let key_usage = KeyUsage :: new () . digital_signature () . key_encipherment () . build () . unwrap () ; extensions . push (key_usage) . unwrap () ; let subject_alternative_name = SubjectAlternativeName :: new () . dns ("example.com") . build (& builder . x509v3_context (None)) . unwrap () ; extensions . push (subject_alternative_name) . unwrap () ; builder . add_extensions (& extensions) . unwrap () ; builder . sign (& pkey , MessageDigest :: sha256 ()) . unwrap () ; let req = builder . build () ; assert ! (req . public_key () . unwrap () . public_eq (& pkey)) ; assert_eq ! (req . extensions () . unwrap () . len () , extensions . len ()) ; assert ! (req . verify (& pkey) . unwrap ()) ; }
};
}
