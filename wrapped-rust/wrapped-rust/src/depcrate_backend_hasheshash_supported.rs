// Generated macro for hash_supported (function)
macro_rules! Depcrate_backend_hasheshash_supported {
() => {
// Module: crate::backend::hashes
// Provides: {"hash_supported"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn hash_supported (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny >) -> bool { message_digest_from_algorithm (py , & algorithm) . is_ok () }
};
}
