// Generated macro for get_rand_bytes (function)
macro_rules! Depcrate_backend_randget_rand_bytes {
() => {
// Module: crate::backend::rand
// Provides: {"get_rand_bytes"}
// Dependencies: {}
pub (crate) fn get_rand_bytes (py : pyo3 :: Python < '_ > , size : usize ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyBytes > > { Ok (pyo3 :: types :: PyBytes :: new_with (py , size , | b | { cryptography_openssl :: rand :: rand_bytes (b) . map_err (CryptographyError :: from) ? ; Ok (()) }) ?) }
};
}
