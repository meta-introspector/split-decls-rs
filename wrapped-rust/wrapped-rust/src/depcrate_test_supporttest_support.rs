// Generated macro for test_support (module)
macro_rules! Depcrate_test_supporttest_support {
() => {
// Module: crate::test_support
// Provides: {"test_support"}
// Dependencies: {}
# [pyo3 :: pymodule (gil_used = false)] pub (crate) mod test_support { # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pymodule_export] use super :: pkcs7_verify ; # [pymodule_export] use super :: test_parse_certificate ; }
};
}
