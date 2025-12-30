// Generated macro for test_parse_certificate (function)
macro_rules! Depcrate_test_supporttest_parse_certificate {
() => {
// Module: crate::test_support
// Provides: {"test_parse_certificate"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn test_parse_certificate (data : & [u8]) -> CryptographyResult < TestCertificate > { let cert = asn1 :: parse_single :: < Certificate < '_ > > (data) ? ; Ok (TestCertificate { not_before_tag : time_tag (& cert . tbs_cert . validity . not_before) , not_after_tag : time_tag (& cert . tbs_cert . validity . not_after) , issuer_value_tags : parse_name_value_tags (& cert . tbs_cert . issuer) , subject_value_tags : parse_name_value_tags (& cert . tbs_cert . subject) , }) }
};
}
