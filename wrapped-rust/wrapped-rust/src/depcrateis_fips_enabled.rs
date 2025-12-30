// Generated macro for is_fips_enabled (function)
macro_rules! Depcrateis_fips_enabled {
() => {
// Module: crate
// Provides: {"is_fips_enabled"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn is_fips_enabled () -> bool { cryptography_openssl :: fips :: is_enabled () }
};
}
