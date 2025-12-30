// Generated macro for openssl_version (function)
macro_rules! Depcrateopenssl_version {
() => {
// Module: crate
// Provides: {"openssl_version"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn openssl_version () -> i64 { openssl :: version :: number () }
};
}
