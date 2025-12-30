// Generated macro for TestCertificate (struct)
macro_rules! Depcrate_test_supportTestCertificate {
() => {
// Module: crate::test_support
// Provides: {"TestCertificate"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.test_support")] struct TestCertificate { # [pyo3 (get)] not_before_tag : u8 , # [pyo3 (get)] not_after_tag : u8 , # [pyo3 (get)] issuer_value_tags : Vec < u8 > , # [pyo3 (get)] subject_value_tags : Vec < u8 > , }
};
}
